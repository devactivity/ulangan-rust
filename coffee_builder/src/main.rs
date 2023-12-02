use coffee_builder::{Coffee, CoffeeBuilder};

fn main() {
    let coffee = CoffeeBuilder::new()
        .set_black_coffee()
        .with_sugar("Regular".to_string())
        .with_milk(3.1)
        .build();
    println!("{:?}", coffee);

    let coffee: Coffee = CoffeeBuilder::new().set_antoccino_coffee().build();
    println!("{:?}", coffee);

    let coffee = CoffeeBuilder::new().set_cubano_coffee().build();
    println!("{:?}", coffee);
}
