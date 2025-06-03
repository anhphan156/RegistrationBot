#[macro_export]
macro_rules! log {
    (let $i:ident = $v:expr;) => {
        let $i = $v;
        println!("{} is {} at {} on {}", stringify!($i), $v, file!(), line!());
    };
}
