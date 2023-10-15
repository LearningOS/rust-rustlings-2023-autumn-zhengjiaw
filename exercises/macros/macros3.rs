// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
    // #[macro_export] 属性不仅使得宏在其定义的模块之外可见，还使得宏在整个 crate 级别可见。
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
// rust 2021 之后
// #![feature(decl_macro)]

// mod macros {
//     pub(crate) macro my_macro() {
//         println!("Check out my macro!");
//     }
// }

// fn main() {
//     macros::my_macro!();
// }