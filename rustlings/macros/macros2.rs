fn main() {
     my_macro!();
}
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
