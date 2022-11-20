#[macro_export]
macro_rules! run {
    ( $op:expr, $input:expr ) => {{
        let result = $op($input.lines());
        println!("{} {}: {}", module_path!(), std::stringify!($op), result);
    }};
}

#[macro_export]
macro_rules! verify {
    ( $op:expr, $input:expr, $expected:expr ) => {{
        let result = $op($input.lines());
        assert_eq!(
            result,
            $expected,
            "{} {}",
            module_path!(),
            std::stringify!($op)
        );
    }};
}
