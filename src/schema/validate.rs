use crate::schema::model::{Config, Schema};

pub struct Validator;

impl Validator {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub fn validate(&self, schema: &Schema) -> Result<(), String> {
        if schema.version < 1 {
            return Err(String::from("schema version is large 0"));
        }

        if schema.config.lang.is_empty() {
            return Err(String::from("empty lang"));
        }

        if ["php", "go"].contains(&schema.config.lang.as_str()) {
            return Err(String::from("unknown lang"));
        }

        if schema.entities.is_empty() {
            return Err(String::from("empty entities"));
        }

        for entity in &schema.entities {
            if entity.name.is_empty() {
                return Err(String::from("empty name"));
            }

            if entity.table.is_empty() {
                return Err(String::from("empty name"));
            }

            if entity.fields.is_empty() {
                return Err(String::from("empty name"));
            }

            for field in &entity.fields {
                if field.name.is_empty() {
                    return Err(String::from("empty name"));
                }

                if field.lang_type.is_empty() {
                    return Err(String::from("lang_type name"));
                }
            }
        }

        Ok(())
    }
}
