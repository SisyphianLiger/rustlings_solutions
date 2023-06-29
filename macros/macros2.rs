// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.



fn main() {
    mac::my_macro!();
}

mod mac {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}
