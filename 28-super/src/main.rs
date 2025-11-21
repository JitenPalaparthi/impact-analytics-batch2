pub mod greet;
fn main() {
    println!("Hello, world!");
    greet::greet::Greet();
    greet::g1::g1::Greet();
    greet::greet::SayHi();
    greet::Greetings::Greet();
}
