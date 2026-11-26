pub struct Schema {
    version: i32,
    config: Config,
    entities: Vec<Entities>,
    relations: Vec<Relation>,
}

pub struct Config {
    lang: String,
    storage: String,
}

pub struct Entities {
    name: String,
    table: String,
    fields: Vec<Field>,
}

pub struct Field {
    name: String,
    lang_type: String,
    required: Option<bool>,
}

pub struct Relation {
    name: String,
    kind: String,
    source: String,
    target: String,
    foreign_key: String,
}
