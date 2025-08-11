use anyhow::{anyhow, Result};
use clap::Parser;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use serde_json::{Value};
use std::{collections::{BTreeMap, HashMap, HashSet}, fs, path::PathBuf};
use regex::Regex;
use chrono::{DateTime, Utc};

#[derive(Parser, Debug)]
#[command(version, about = "Generate JSON from .jgd definitions")]
struct Cli {
    /// Path to .jgd file
    input: PathBuf,
    /// Entity to generate (may be repeated). If omitted, generates all (entities mode only).
    #[arg(short, long)]
    entity: Vec<String>,
    /// Output file (JSON). If omitted, prints to stdout.
    #[arg(short, long)]
    out: Option<PathBuf>,
    /// Emit NDJSON (one JSON per line)
    #[arg(long)]
    ndjson: bool,
    /// Seed override
    #[arg(long)]
    seed: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Jgd {
    #[serde(rename = "$format")]
    format: String,
    version: String,
    #[serde(default)] seed: Option<u64>,
    #[serde(default = "default_locale")]
    default_locale: String,
    #[serde(default)] entities: Option<BTreeMap<String, Entity>>, // collection mode
    #[serde(default)] root: Option<RootEntity>,                    // root mode
}
fn default_locale() -> String { "EN".to_string() }

#[derive(Debug, Deserialize)]
struct Entity {
    count: Count,
    #[serde(default)] seed: Option<u64>,
    #[serde(default)]
    unique_by: Vec<String>,
    fields: BTreeMap<String, Field>,
}

#[derive(Debug, Deserialize)]
struct RootEntity {
    #[serde(default)] count: Option<Count>,
    #[serde(default)] seed: Option<u64>,
    #[serde(default)]
    unique_by: Vec<String>,
    fields: BTreeMap<String, Field>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Count { Fixed(u64), Range((u64,u64)) }

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Field {
    Faker {
        faker: String, #[serde(default)] locale: Option<String>,
        #[serde(default)] between: Option<(String,String)>,
        #[serde(default)] tz: Option<String>,
        #[serde(default)] words: Option<(u64,u64)>,
        #[serde(default)] count: Option<u64>
    },
    Object { object: BTreeMap<String, Field> },
    Array  { array: ArraySpec },
    Number { number: NumberSpec },
    Optional { optional: OptionalSpec },
    Template { template: String },
    Ref { r#ref: String },
    Str(String), Bool(bool), I64(i64), F64(f64), Null,
}

#[derive(Debug, Deserialize, Clone)]
struct ArraySpec { of: Box<Field>, #[serde(default)] length: Option<Len> }

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Len { Fixed(u64), Range((u64,u64)) }

#[derive(Debug, Deserialize, Clone)]
struct NumberSpec { min: f64, max: f64, #[serde(default)] integer: bool }

#[derive(Debug, Deserialize, Clone)]
struct OptionalSpec { of: Box<Field>, #[serde(default = "default_prob")] prob: f64 }
fn default_prob() -> f64 { 0.5 }

fn open_and_print(path: &str) {
    let value = jgd_lib::from_file(path);
    let serialized = serde_json::to_string_pretty(&value).unwrap();
    println!("\nFile: {}", path);
    println!("---------------------------------------------------\n");
    println!("serialized = {}", serialized);
}

fn main() -> Result<()> {
    open_and_print("/Users/lvendrame/projects/pocs/jgd-rs/jgd-lib/examples/single-object-root.jgd");
    open_and_print("/Users/lvendrame/projects/pocs/jgd-rs/jgd-lib/examples/array-object-root.jgd");
    open_and_print("/Users/lvendrame/projects/pocs/jgd-rs/jgd-lib/examples/ranged-array-object-root.jgd");
    //open_and_print("/Users/lvendrame/projects/pocs/jgd-rs/jgd-lib/examples/user-post-entities.jgd");
    return Ok(());

    let cli = Cli::parse();

    //jgd_lib::from_file(&cli.input);

    let raw = fs::read_to_string(&cli.input)?;
    let jgd: Jgd = serde_json::from_str(&raw)?;

    // Root mode
    if let Some(root) = &jgd.root {
        if !cli.entity.is_empty() { return Err(anyhow!("--entity cannot be used with root mode")); }
        let seed = cli.seed.or(jgd.seed).or(root.seed).unwrap_or(0);
        let mut rng = StdRng::seed_from_u64(seed);
    let default_locale = &jgd.default_locale;

        let mut items = Vec::new();
        let count = match root.count.clone().unwrap_or(Count::Fixed(1)) {
            Count::Fixed(n) => n,
            Count::Range((a,b)) => rng.random_range(a..=b),
        };

        let mut unique_sets: HashMap<String, HashSet<String>> = HashMap::new();
        let mut ent_rng = StdRng::seed_from_u64(root.seed.unwrap_or(seed));
        for _ in 0..count {
            let obj = gen_object(&root.fields, default_locale, &mut ent_rng, &HashMap::new(), "root")?;
            if !root.unique_by.is_empty() {
                let fp = fingerprint(&obj, &root.unique_by);
                let set = unique_sets.entry(root.unique_by.join("|"))
                    .or_default();
                if set.contains(&fp) {
                    continue;
                } else {
                    set.insert(fp);
                }
            }
            items.push(serde_json::Value::Object(obj.into_iter().collect()));
        }

        let out = if cli.ndjson {
            let mut s = String::new();
            for it in &items {
                s.push_str(&serde_json::to_string(it)?);
                s.push('\n');
            }
            s
        } else if root.count.is_none() && items.len() == 1 {
            serde_json::to_string_pretty(&items[0])?
        } else {
            serde_json::to_string_pretty(&items)?
        };

        if let Some(p) = cli.out { fs::write(p, out)?; } else { println!("{}", out); }
        return Ok(());
    }

    // Entities mode
    let entities = jgd.entities.ok_or_else(|| anyhow!("either 'entities' or 'root' must be provided"))?;
    let seed = cli.seed.or(jgd.seed).unwrap_or(0);
    let mut rng = StdRng::seed_from_u64(seed);

    let targets: Vec<String> = if cli.entity.is_empty() {
        entities.keys().cloned().collect()
    } else {
        cli.entity.clone()
    };

    let mut store: HashMap<String, Vec<Value>> = HashMap::new();

    for (name, ent) in &entities {
        if !targets.contains(name) { continue; }
        let count = match ent.count {
            Count::Fixed(n) => n,
            Count::Range((a, b)) => rng.random_range(a..=b),
        };

        let mut items = Vec::with_capacity(count as usize);
        let mut unique_sets: HashMap<String, HashSet<String>> = HashMap::new();
        let mut ent_rng = StdRng::seed_from_u64(ent.seed.unwrap_or(seed));
        for _ in 0..count {
            let obj = gen_object(&ent.fields, &jgd.default_locale, &mut ent_rng, &store, name)?;
            if !ent.unique_by.is_empty() {
                let fp = fingerprint(&obj, &ent.unique_by);
                let set = unique_sets.entry(ent.unique_by.join("|"))
                    .or_default();
                if set.contains(&fp) {
                    continue;
                } else {
                    set.insert(fp);
                }
            }
            items.push(serde_json::Value::Object(obj.into_iter().collect()));
        }
        store.insert(name.clone(), items);
    }

    // Emit
    let out = if cli.ndjson {
        let mut s = String::new();
        for t in &targets {
            if let Some(v) = store.get(t) {
                for item in v {
                    s.push_str(&serde_json::to_string(item)?);
                    s.push('\n');
                }
            }
        }
        s
    } else if targets.len() == 1 {
        let t = &targets[0];
        serde_json::to_string_pretty(store.get(t).unwrap_or(&Vec::new()))?
    } else {
        let mut obj = serde_json::Map::new();
        for t in &targets {
            obj.insert(t.clone(), Value::Array(store.get(t).cloned().unwrap_or_default()));
        }
        serde_json::to_string_pretty(&Value::Object(obj))?
    };

    if let Some(p) = cli.out { fs::write(p, out)?; } else { println!("{}", out); }
    Ok(())
}

fn gen_object(fields: &BTreeMap<String, Field>, default_locale: &str, rng: &mut StdRng, store: &HashMap<String, Vec<Value>>, cur_entity: &str) -> Result<BTreeMap<String, Value>> {
    let mut ctx: BTreeMap<String, Value> = BTreeMap::new();
    // first pass
    for (k, spec) in fields {
        let v = gen_field(spec, default_locale, rng, store, cur_entity)?;
        ctx.insert(k.clone(), v);
    }
    // template post-pass (supports {field} and nested paths like {address.city})
    let ctx_copy = ctx.clone();
    let mut out = BTreeMap::new();
    for (k, v) in ctx.into_iter() {
        if let Value::String(s) = &v {
            out.insert(k, Value::String(apply_template(s, &ctx_copy)));
        } else {
            out.insert(k, v);
        }
    }
    Ok(out)
}

fn fingerprint(obj: &BTreeMap<String, Value>, keys: &[String]) -> String {
    let mut parts = Vec::new();
    for k in keys { if let Some(v) = obj.get(k) { parts.push(v.to_string()); } }
    parts.join("|")
}

fn apply_template(s: &str, ctx: &BTreeMap<String, Value>) -> String {
    let re = Regex::new(r"\{([a-zA-Z0-9_.-]+)\}").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let key = &caps[1];
        // support nested path lookups
        lookup_path(ctx, key).and_then(|v| v.as_str().map(|x| x.to_string())).unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
    }).to_string()
}

fn lookup_path<'a>(ctx: &'a BTreeMap<String, Value>, path: &str) -> Option<&'a Value> {
    let mut cur: Option<&Value> = None;
    for (i, part) in path.split('.').enumerate() {
        cur = if i == 0 { ctx.get(part) } else { cur.and_then(|v| v.get(part)) };
        if cur.is_none() {
            return None;
        }
    }
    cur
}

fn gen_field(spec: &Field, default_locale: &str, rng: &mut StdRng, store: &HashMap<String, Vec<Value>>, _cur_entity: &str) -> Result<Value> {
    // use fake::{Fake, faker};
    match spec {
        Field::Str(s) => Ok(Value::String(s.clone())),
        Field::Bool(b) => Ok(Value::Bool(*b)),
        Field::I64(n) => Ok(Value::Number((*n).into())),
        Field::F64(n) => Ok(serde_json::Number::from_f64(*n).map(Value::Number).ok_or_else(|| anyhow!("NaN"))?),
        Field::Null => Ok(Value::Null),
        Field::Number { number } => {
            if number.integer {
                Ok(Value::from(rng.random_range(number.min as i64 ..= number.max as i64)))
            } else {
                Ok(Value::from(rng.random_range(number.min..=number.max)))
            }
        }
        Field::Array { array } => {
            let len = match array.length.clone().unwrap_or(Len::Fixed(1)) {
                Len::Fixed(n) => n,
                Len::Range((a, b)) => rng.random_range(a..=b),
            };
            let mut arr = Vec::with_capacity(len as usize);
            for _ in 0..len {
                arr.push(gen_field(&array.of, default_locale, rng, store, _cur_entity)?);
            }
            Ok(Value::Array(arr))
        }
        Field::Object { object } => {
            let mut map = serde_json::Map::new();
            for (k, v) in object { map.insert(k.clone(), gen_field(v, default_locale, rng, store, _cur_entity)?); }
            Ok(Value::Object(map))
        }
        Field::Optional { optional } => {
            if rng.random::<f64>() < optional.prob {
                gen_field(&optional.of, default_locale, rng, store, _cur_entity)
            } else {
                Ok(Value::Null)
            }
        }
        Field::Template { template } => Ok(Value::String(template.clone())), // post-processed later
        Field::Ref { r#ref } => {
            // form: entity.path.to.field
            let mut parts = r#ref.split('.');
            let ent = parts.next().unwrap();
            let keypath: Vec<&str> = parts.collect();
            let pool = store.get(ent).ok_or_else(|| anyhow!("unknown ref entity {ent}"))?;
            if pool.is_empty() {
                return Err(anyhow!("ref entity {ent} is empty"));
            }
            let idx = rng.random_range(0..pool.len());
            let mut v = pool[idx].clone();
            for k in keypath {
                v = v.get(k).cloned().ok_or_else(|| anyhow!("ref path not found: {}", k))?;
            }
            Ok(v)
        }
        Field::Faker { faker, locale, between, tz: _tz, words, count: _ } => {
            let loc = locale.as_deref().unwrap_or(default_locale);
            Ok(dispatch_faker(faker, loc, *words, between.clone(), rng)?)
        }
    }
}

fn dispatch_faker(faker_path: &str, locale: &str, words: Option<(u64,u64)>, between: Option<(String,String)>, rng: &mut StdRng) -> Result<Value> {
    use fake::{Fake, faker};
    match faker_path {
        // name.name
        "name.name" => {
            match locale {
                "EN" => Ok(Value::String(faker::name::en::Name().fake_with_rng(rng))),
                "FR_FR" => Ok(Value::String(faker::name::fr_fr::Name().fake_with_rng(rng))),
                "IT_IT" => Ok(Value::String(faker::name::it_it::Name().fake_with_rng(rng))),
                "JA_JP" => Ok(Value::String(faker::name::ja_jp::Name().fake_with_rng(rng))),
                "DE_DE" => Ok(Value::String(faker::name::de_de::Name().fake_with_rng(rng))),
                "PT_BR" => Ok(Value::String(faker::name::pt_br::Name().fake_with_rng(rng))),
                "AR_SA" => Ok(Value::String(faker::name::ar_sa::Name().fake_with_rng(rng))),
                "CY_GB" => Ok(Value::String(faker::name::cy_gb::Name().fake_with_rng(rng))),
                _ => Ok(Value::String(faker::name::raw::Name(fake::locales::EN).fake_with_rng(rng)))
            }
        }
        // internet.safeEmail
        "internet.safeEmail" => {
            match locale {
                "EN" => Ok(Value::String(faker::internet::en::SafeEmail().fake_with_rng(rng))),
                "FR_FR" => Ok(Value::String(faker::internet::fr_fr::SafeEmail().fake_with_rng(rng))),
                "IT_IT" => Ok(Value::String(faker::internet::it_it::SafeEmail().fake_with_rng(rng))),
                "JA_JP" => Ok(Value::String(faker::internet::ja_jp::SafeEmail().fake_with_rng(rng))),
                "DE_DE" => Ok(Value::String(faker::internet::de_de::SafeEmail().fake_with_rng(rng))),
                "PT_BR" => Ok(Value::String(faker::internet::pt_br::SafeEmail().fake_with_rng(rng))),
                "AR_SA" => Ok(Value::String(faker::internet::ar_sa::SafeEmail().fake_with_rng(rng))),
                "CY_GB" => Ok(Value::String(faker::internet::cy_gb::SafeEmail().fake_with_rng(rng))),
                _ => Ok(Value::String(faker::internet::raw::SafeEmail(fake::locales::EN).fake_with_rng(rng)))
            }
        }
        // address.city
        "address.city" => {
            match locale {
                "EN" => Ok(Value::String(faker::address::en::CityName().fake_with_rng(rng))),
                "FR_FR" => Ok(Value::String(faker::address::fr_fr::CityName().fake_with_rng(rng))),
                "IT_IT" => Ok(Value::String(faker::address::it_it::CityName().fake_with_rng(rng))),
                "JA_JP" => Ok(Value::String(faker::address::ja_jp::CityName().fake_with_rng(rng))),
                "DE_DE" => Ok(Value::String(faker::address::de_de::CityName().fake_with_rng(rng))),
                "PT_BR" => Ok(Value::String(faker::address::pt_br::CityName().fake_with_rng(rng))),
                "AR_SA" => Ok(Value::String(faker::address::ar_sa::CityName().fake_with_rng(rng))),
                "CY_GB" => Ok(Value::String(faker::address::cy_gb::CityName().fake_with_rng(rng))),
                _ => Ok(Value::String(faker::address::raw::CityName(fake::locales::EN).fake_with_rng(rng)))
            }
        }
        // address.city
        "address.zipCode" => {
            match locale {
                "EN" => Ok(Value::String(faker::address::en::ZipCode().fake_with_rng(rng))),
                "FR_FR" => Ok(Value::String(faker::address::fr_fr::ZipCode().fake_with_rng(rng))),
                "IT_IT" => Ok(Value::String(faker::address::it_it::ZipCode().fake_with_rng(rng))),
                "JA_JP" => Ok(Value::String(faker::address::ja_jp::ZipCode().fake_with_rng(rng))),
                "DE_DE" => Ok(Value::String(faker::address::de_de::ZipCode().fake_with_rng(rng))),
                "PT_BR" => Ok(Value::String(faker::address::pt_br::ZipCode().fake_with_rng(rng))),
                "AR_SA" => Ok(Value::String(faker::address::ar_sa::ZipCode().fake_with_rng(rng))),
                "CY_GB" => Ok(Value::String(faker::address::cy_gb::ZipCode().fake_with_rng(rng))),
                _ => Ok(Value::String(faker::address::raw::CityName(fake::locales::EN).fake_with_rng(rng)))
            }
        }
        // lorem.word / lorem.sentence
        "lorem.word" => {
            match locale {
                "EN" => Ok(Value::String(faker::lorem::en::Word().fake_with_rng(rng))),
                "FR_FR" => Ok(Value::String(faker::lorem::fr_fr::Word().fake_with_rng(rng))),
                "IT_IT" => Ok(Value::String(faker::lorem::it_it::Word().fake_with_rng(rng))),
                "JA_JP" => Ok(Value::String(faker::lorem::ja_jp::Word().fake_with_rng(rng))),
                "DE_DE" => Ok(Value::String(faker::lorem::de_de::Word().fake_with_rng(rng))),
                "PT_BR" => Ok(Value::String(faker::lorem::pt_br::Word().fake_with_rng(rng))),
                "AR_SA" => Ok(Value::String(faker::lorem::ar_sa::Word().fake_with_rng(rng))),
                "CY_GB" => Ok(Value::String(faker::lorem::cy_gb::Word().fake_with_rng(rng))),
                _ => Ok(Value::String(faker::lorem::raw::Word(fake::locales::EN).fake_with_rng(rng)))
            }
        }
        "lorem.sentence" => {
            let (a, b) = words.unwrap_or((3, 7));
            let range = a as usize .. b as usize + 1;
            let s: String = match locale {
                "EN" => faker::lorem::en::Sentence(range).fake_with_rng(rng),
                "FR_FR" => faker::lorem::fr_fr::Sentence(range).fake_with_rng(rng),
                "IT_IT" => faker::lorem::it_it::Sentence(range).fake_with_rng(rng),
                "JA_JP" => faker::lorem::ja_jp::Sentence(range).fake_with_rng(rng),
                "DE_DE" => faker::lorem::de_de::Sentence(range).fake_with_rng(rng),
                "PT_BR" => faker::lorem::pt_br::Sentence(range).fake_with_rng(rng),
                "AR_SA" => faker::lorem::ar_sa::Sentence(range).fake_with_rng(rng),
                "CY_GB" => faker::lorem::cy_gb::Sentence(range).fake_with_rng(rng),
                _ => faker::lorem::en::Sentence(range).fake_with_rng(rng)
            };
            Ok(Value::String(s))
        }
        // uuid.v4
        "uuid.v4" => {
            let id = uuid::Uuid::new_v4();
            Ok(Value::String(id.to_string()))
        }
        // ulid
        "ulid" => {
            let id = ulid::Ulid::new();
            Ok(Value::String(id.to_string()))
        }
        // chrono.datetime with between
        "chrono.datetime" => {
            let (a,b) = between.ok_or_else(|| anyhow!("chrono.datetime requires 'between'"))?;
            let start: DateTime<Utc> = a.parse().map_err(|_| anyhow!("invalid datetime: {a}"))?;
            let end: DateTime<Utc> = b.parse().map_err(|_| anyhow!("invalid datetime: {b}"))?;
            if end <= start { return Err(anyhow!("between start must be < end")); }
            let span = (end - start).num_milliseconds();
            let offset = rng.random_range(0..span);
            let ts = start + chrono::Duration::milliseconds(offset);
            Ok(Value::String(ts.to_rfc3339()))
        }
        _ => Err(anyhow!("unsupported faker: {faker_path}"))
    }
}
