use crate::schema::model::Schema;

pub trait Generator {
    fn generate(schema: &Schema) -> Result<(), String>;
}
