use crate::schema::model::Schema;
use garde::Validate;

const SUPPORTED_LANGUAGES: [&str; 2] = ["php", "go"];

pub struct Validator;

impl Validator {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn validate(&self, schema: &Schema) -> Result<(), String> {
        if schema.version < 1 {
            return Err(String::from("schema version must be greater than 0"));
        }

        if !SUPPORTED_LANGUAGES.contains(&schema.config.lang.as_str()) {
            return Err(format!("unknown lang: {}", schema.config.lang));
        }

        schema
            .validate()
            .map_err(|error| format!("schema: {error}"))?;

        schema
            .config
            .validate()
            .map_err(|error| format!("config: {error}"))?;

        for (entity_index, entity) in schema.entities.iter().enumerate() {
            entity
                .validate()
                .map_err(|error| format!("entities[{entity_index}]: {error}"))?;

            for (field_index, field) in entity.fields.iter().enumerate() {
                field.validate().map_err(|error| {
                    format!("entities[{entity_index}].fields[{field_index}]: {error}")
                })?;
            }
        }

        Ok(())
    }
}
