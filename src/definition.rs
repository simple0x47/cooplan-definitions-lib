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

impl PartialEq for Definition {
    fn eq(&self, other: &Self) -> bool {
        if self.version != other.version {
            return false;
        }

        if self.categories.len() != other.categories.len() {
            return false;
        }

        for i in 0..self.categories.len() {
            if self.categories[i] != other.categories[i] {
                return false;
            }
        }

        true
    }
}
