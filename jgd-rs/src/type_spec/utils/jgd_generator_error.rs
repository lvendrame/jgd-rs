use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct JgdGeneratorError {
    pub message: String,
    pub entity: Option<String>,
    pub field: Option<String>,
}

impl Display for JgdGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.entity, &self.field) {
            (Some(entity), Some(field)) => write!(f, "{} (entity: {}, field: {})", self.message, entity, field),
            (Some(entity), None) => write!(f, "{} (entity: {})", self.message, entity),
            (None, Some(field)) => write!(f, "{} (field: {})", self.message, field),
            (None, None) => write!(f, "{}", self.message),
        }
    }
}
