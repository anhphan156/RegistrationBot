#[macro_export]
macro_rules! log_assignment {
    (let $i:ident = $v:expr;) => {
        let $i = $v;
        println!("{} is {} at {} on {}", stringify!($i), $v, file!(), line!());
    };
}

#[macro_export]
macro_rules! log_expression {
    ($v:expr) => {
        println!("{} at {} on {}", $v, file!(), line!());
    }
}
