use crate::generator::generator::Generator;
use crate::schema::model::Schema;

pub struct Go;

impl Generator for Go {

    fn generate(schema: &Schema) -> Result<(), String> {
        todo!()
    }
}