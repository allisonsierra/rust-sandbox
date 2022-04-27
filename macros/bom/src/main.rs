use bom::model::{Category, Component};
use validate::validate::Validate;

fn main() {
    let component = Component::Unknown("tacos".to_string());

    match component.validate() {
        Ok(result) => println!("{:?}", result),
        Err(_) => print!("Component validate failed"),
    }

    let category = Category::Unknown("burritos".to_string());

    match category.validate() {
        Ok(result) => println!("{:?}", result),
        Err(_) => print!("Category validate failed"),
    }
}
