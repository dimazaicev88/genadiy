use crate::generator::generator::Generator;
use crate::schema::model::Schema;

pub struct Php;

impl Generator for Php {
    fn generate(schema: &Schema) -> Result<(), String> {
        todo!()
    }
}