use crate::source_category::SourceCategory;
use serde::{Deserialize, Serialize};

use crate::validated_source_attribute::ValidatedSourceAttribute;

/// Full, input based category.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatedSourceCategory {
    pub id: String,
    pub parent: Option<String>,
    pub parent_name: Option<String>,
    pub name: String,
    pub selectable_as_last: bool,
    pub attributes: Vec<ValidatedSourceAttribute>,
}

impl TryFrom<SourceCategory> for ValidatedSourceCategory {
    type Error = String;

    fn try_from(value: SourceCategory) -> Result<Self, Self::Error> {
        match &value.id {
            Some(id) => match ValidatedSourceAttribute::validate_attributes(value.attributes) {
                Ok(validated_attributes) => Ok(ValidatedSourceCategory {
                    id: id.clone(),
                    parent: value.parent,
                    parent_name: value.parent_name,
                    name: value.name,
                    selectable_as_last: value.selectable_as_last.unwrap_or(false),
                    attributes: validated_attributes,
                }),
                Err(error) => Err(String::from(format!(
                    "failed to validate attributes: {}",
                    error
                ))),
            },
            None => Err(String::from(format!(
                "source category '{}' has no id",
                value.name
            ))),
        }
    }
}

impl PartialEq for ValidatedSourceCategory {
    fn eq(&self, other: &Self) -> bool {
        if !(self.id == other.id
            && self.parent == other.parent
            && self.parent_name == other.parent_name
            && self.name == other.name
            && self.selectable_as_last == other.selectable_as_last)
        {
            return false;
        }

        if self.attributes.len() != other.attributes.len() {
            return false;
        }

        for i in 0..self.attributes.len() {
            if self.attributes[i] != other.attributes[i] {
                return false;
            }
        }

        true
    }
}
