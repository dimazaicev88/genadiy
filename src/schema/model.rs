use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Validate, Deserialize)]
pub struct Schema {
    #[garde(skip)]
    pub version: i32,

    #[garde(skip)]
    pub config: Config,

    #[garde(length(min = 1))]
    pub entities: Vec<Entities>,

    #[garde(skip)]
    pub relations: Option<Vec<Relation>>,
}

#[derive(Debug, Default, Clone, Validate, Deserialize)]
pub struct Config {
    #[garde(length(min = 2))]
    pub lang: String,

    #[garde(skip)]
    pub storage: Storage,
}

#[derive(Debug, Default, Clone, Validate, Deserialize)]
pub struct Entities {
    #[garde(length(min = 2))]
    pub name: String,

    #[garde(length(min = 2))]
    pub table: String,

    #[garde(length(min = 1))]
    pub fields: Vec<Field>,
}

#[derive(Debug, Default, Clone, Validate, Deserialize)]
pub struct Field {
    #[garde(length(min = 2))]
    pub name: String,

    #[garde(length(min = 2))]
    pub lang_type: String,

    #[garde(skip)]
    pub required: Option<bool>,
}

#[derive(Debug, Default, Clone, Validate, Deserialize)]
pub struct Relation {
    #[garde(length(min = 2))]
    pub name: String,

    #[garde(length(min = 2))]
    pub kind: String,

    #[garde(length(min = 2))]
    pub source: String,

    #[garde(length(min = 2))]
    pub target: String,

    #[garde(length(min = 2))]
    pub foreign_key: String,
}

#[derive(Debug, Default, Clone, Validate, Deserialize)]
pub struct Storage {
    #[garde(length(min = 4))]
    pub engine: String,

    #[garde(length(min = 1))]
    pub login: String,

    #[garde(length(min = 1))]
    pub password: String,

    #[garde(length(min = 1))]
    pub host: String,

    #[garde(length(min = 1))]
    pub db_name: String,

    #[garde(length(min = 3))]
    pub charset: String,
}
