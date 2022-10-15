use crate::validated_source_category::ValidatedSourceCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    version: String,
    categories: Vec<ValidatedSourceCategory>,
}

impl Definition {
    pub fn new(version: String, categories: Vec<ValidatedSourceCategory>) -> Definition {
        Definition {
            version,
            categories,
        }
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn categories(&self) -> Vec<ValidatedSourceCategory> {
        self.categories.clone()
    }
}
