use upon::Error;
use crate::generator::templates::TEMPLATE_DB_CONNECT;
use crate::schema::model::Storage;

pub struct DbGenerator;

impl DbGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn db_generate(&self, storage: &Storage) -> Result<String, Error> {
        let template_name = "db_conn";
        let mut engine = upon::Engine::new();
        engine.add_template(template_name, TEMPLATE_DB_CONNECT)?;

        let result = engine
            .template(template_name)
            .render(upon::value! {
                php: {
                    namespace: "John Smith" //TODO
                },
                db: {
                    db_host: &storage.login,
                    db_name: &storage.db_name,
                    db_charset: &storage.charset,
                    db_login: &storage.login,
                    db_password: &storage.password
                }
            })
            .to_string()?;

        Ok(result)
    }
}