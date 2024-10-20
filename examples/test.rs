use concat_mulidents::concat_idents;
macro_rules! concat_test {
    ($name:ident, $name2:ident) => {
        concat_idents! {
            fn $name--_--$name2() {
                let $name--$name2 = "Hello, World!";
                println!("{}", $name--$name2);
            }
        }
    };
}

fn main () {
    concat_test!(hello, world);
    hello_world();
}