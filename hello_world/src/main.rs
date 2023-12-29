use std::io;

fn hello_world() {
    println!("hahahhaha");
}

fn main() {
    println!("Hello, world!");
    hello_world();

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法");

    println!("你猜测的数{}", guess);

}
