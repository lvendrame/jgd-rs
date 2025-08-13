pub enum LocalesKeys {
    En,
    FrFr,
    ItIt,
    JaJp,
    DeDe,
    PtBr,
    ArSa,
    CyGb,
}

impl From<LocalesKeys> for &str {
    fn from(value: LocalesKeys) -> Self {
        match value {
            LocalesKeys::En => "EN",
            LocalesKeys::FrFr => "FR_FR",
            LocalesKeys::ItIt => "IT_IT",
            LocalesKeys::JaJp => "JA_JP",
            LocalesKeys::DeDe => "DE_DE",
            LocalesKeys::PtBr => "PT_BR",
            LocalesKeys::ArSa => "AR_SA",
            LocalesKeys::CyGb => "CY_GB",
        }
    }
}

impl From<&str> for LocalesKeys {
    fn from(value: &str) -> Self {
        match value {
            "EN" => LocalesKeys::En,
            "FR_FR" => LocalesKeys::FrFr,
            "IT_IT" => LocalesKeys::ItIt,
            "JA_JP" => LocalesKeys::JaJp,
            "DE_DE" => LocalesKeys::DeDe,
            "PT_BR" => LocalesKeys::PtBr,
            "AR_SA" => LocalesKeys::ArSa,
            "CY_GB" => LocalesKeys::CyGb,
            _ => LocalesKeys::En,
        }
    }
}
