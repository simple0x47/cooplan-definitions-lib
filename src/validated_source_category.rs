use crate::source_category::SourceCategory;
use serde::{Deserialize, Serialize};

use crate::validated_source_attribute::ValidatedSourceAttribute;

/// Full, input based category.
#[derive(Serialize, Deserialize, Debug)]
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
