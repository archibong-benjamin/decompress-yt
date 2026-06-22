// create your own simple macro
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!(); // expands to println!("Hello!") at compile time
}
