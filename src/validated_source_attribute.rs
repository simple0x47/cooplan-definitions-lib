use crate::source_attribute::SourceAttribute;
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

/// Full, input based attribute.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatedSourceAttribute {
    pub id: String,
    pub name: String,
    pub data_type: String,
    pub unit: Option<String>,
    pub optional: bool,
}

impl ValidatedSourceAttribute {
    pub fn validate_attributes(
        source_attributes: Vec<SourceAttribute>,
    ) -> Result<Vec<ValidatedSourceAttribute>, Error> {
        let mut validated_attributes: Vec<ValidatedSourceAttribute> = Vec::new();

        for source_attribute in source_attributes {
            match ValidatedSourceAttribute::try_from(source_attribute) {
                Ok(validated_attribute) => validated_attributes.push(validated_attribute),
                Err(error) => {
                    return Err(Error::new(
                        ErrorKind::FailedToValidateSourceAttribute,
                        format!("failed to validate source attribute: {}", error).as_str(),
                    ));
                }
            }
        }

        Ok(validated_attributes)
    }
}

impl TryFrom<SourceAttribute> for ValidatedSourceAttribute {
    type Error = String;

    fn try_from(value: SourceAttribute) -> Result<Self, Self::Error> {
        match &value.id {
            Some(id) => Ok(ValidatedSourceAttribute {
                id: id.clone(),
                name: value.name,
                data_type: value.data_type,
                unit: value.unit,
                optional: value.optional.unwrap_or(false),
            }),
            None => Err(String::from(format!(
                "source attribute '{}' has no id",
                value.name
            ))),
        }
    }
}

impl PartialEq for ValidatedSourceAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.data_type == other.data_type
            && self.unit == other.unit
            && self.optional == other.optional
    }
}
