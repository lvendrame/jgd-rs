use rand::rngs::StdRng;

pub struct LocalConfig {
    /// Random number generator for deterministic or random generation.
    ///
    /// Uses `StdRng` to provide high-quality random numbers. Can be seeded for
    /// deterministic generation (useful for testing) or use a random seed for
    /// truly random output.
    pub rng: Option<StdRng>,

    pub entity_name: Option<String>,

    pub field_name: Option<String>,

    pub indices: Vec<usize>,

    pub count_items: u64,
}

impl LocalConfig {
    pub fn new(rng: Option<StdRng>) -> Self {
        Self {
            rng,
            entity_name: None,
            field_name: None,
            indices: vec![],
            count_items: 0,
        }
    }

    pub fn from_current(
        rng: Option<StdRng>,
        count_items: u64,
        entity_name: Option<&str>,
        field_name: Option<&str>,
        index: Option<usize>,
        parents: Option<&[usize]>
    ) -> Self {
        let mut indices = vec![];

        if let Some(index) = index {
            indices.push(index);
        };

        if let Some(parents) = parents {
            indices.extend_from_slice(parents);
        };

        Self {
            rng,
            entity_name: entity_name.map(|v| v.to_string()),
            field_name: field_name.map(|v| v.to_string()),
            indices,
            count_items,
        }
    }

    pub fn from_current_with_config(
        rng: Option<StdRng>,
        count_items: u64,
        parent_config: Option<&mut LocalConfig>,
    ) -> Self {
        if let Some(config) = parent_config {
            let rng = if rng.is_some() {
                rng
            } else {
                config.rng.clone()
            };
            return Self::from_current(
                rng, count_items,
                config.entity_name.as_deref(),
                config.field_name.as_deref(),
                Some(0),
                Some(&config.indices)
            );
        }

        Self::from_current(rng, count_items, None, None, Some(0), None)
    }

    pub fn get_index(&self, depth: usize) -> Option<usize> {
        if depth < self.indices.len() {
            return Some(self.indices[depth])
        }
        None
    }

    pub fn set_index(&mut self, index: usize) {
        if !self.indices.is_empty() {
            self.indices[0] = index;
        }
    }
}
