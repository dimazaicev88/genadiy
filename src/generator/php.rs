use crate::generator::generator::Generator;
use crate::generator::templates::TEMPLATE_DB_CONNECT;
use crate::schema::model::{Schema, Storage};
use upon::Error;

pub struct Php;


impl Generator for Php {
    fn generate(schema: &Schema) -> Result<(), String> {
        todo!()
    }
}


