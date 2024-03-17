fn main() {
    let name = "Toni";
    greet(name);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
