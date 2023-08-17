fn main() {
    enum_code();
    trait_code();
}

fn enum_code() {
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

fn trait_code() {
    let dog = Dog {};
    let cat = Cat {};
    show_animal_data(dog);
    show_animal_data(cat);
}

fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {}", animal.lifespan());
    println!("Scientific name: {}", animal.scientific_name());
}
