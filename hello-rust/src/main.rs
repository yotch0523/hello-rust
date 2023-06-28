fn main() {
    println!("Hello, world!");
    chapter1_enum();
    chapter1_trait();
}

fn chapter1_enum() {
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("this number is even: {}", x),
        Some(x) => println!("this number is odd: {}", x),
        None => print!("none"),
    }
    let mut v = vec![];
    v.push(1);
}

trait Animal {
    fn lifespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }
    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

struct Cat;
impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }
    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

fn chapter1_trait() {
    let dog = Dog{};
    let cat = Cat{};
    println!("dog lifespan: {}", dog.lifespan());
    println!("dog scientific_name: {}", dog.scientific_name());
    println!("cat lifespan: {}", cat.lifespan());
    println!("cat scientific_name: {}", cat.scientific_name());
}