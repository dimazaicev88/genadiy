use std::fs;

pub mod schema;

fn main() {
    let content =
        fs::read_to_string("/home/dima/projects/genadiy/schema.yml").expect("failed to read file");
    let schema = schema::parser::Parser::new(content)
        .parse()
        .expect("error parse file");

    println!("{:?}", schema)
}
