fn main() {
    let dove = Dove {};
    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    // borrow
    let important_data = "Hello, world.".to_string();

    calc_data(&important_data);
    println!("{}", important_data);

    // lifetime
    let mut x = 5;
    let _y = &x; // yのライフタイムがここで終了するので、後続にxの可変参照を渡してもエラーにならない
    let z = &mut x;

    dbg!(z);
    dbg!(x);
}

trait Tweet {
    fn tweet(&self);
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn calc_data(data: &String) {
    println!("{}", data);
}
