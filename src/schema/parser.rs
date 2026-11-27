use crate::schema::model::Schema;
use crate::schema::validate::Validator;

pub struct Parser {
    source: String,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Self { source }
    }

    pub fn parse(&self) -> Result<Schema, String> {
        let config: Schema = serde_saphyr::from_str(self.source.as_str()).expect("failed to parse");
        Validator::new().validate(&config)?;

        Ok(config)
    }
}
