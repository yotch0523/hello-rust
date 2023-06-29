fn main() {
    let mut tuple = (1, "2");
    tuple.0 = 2;
    tuple.1 = "3";

    let mut a = [1, 1, 2];
    let b = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[0..3]);

    let p = Person {
        name: Some(String::from("youki")),
        age: Some(12),
    };

    match p.name {
        Some(x) => println!("name: {}", x),
        None => println!("name is not set"),
    }
    match p.age {
        Some(x) => println!("age: {}", x),
        None => println!("age is not set"),
    }

    let success: Result<i32, String> = Ok(200);
    let error: Result<i32, String> = Err("not found".to_string());

    display_result(success);
    display_result(error);

    let x = [b'Y'];
    prints(Box::new(x));
}

struct Person {
    name: Option<String>,
    age: Option<u32>,
}

fn display_result(result: Result<i32, String>) {
    println!("display_result | code: {}", result.unwrap_or(-1));
}

fn prints(s: Box<[u8]>) {
    println!("{:?}", s);
}