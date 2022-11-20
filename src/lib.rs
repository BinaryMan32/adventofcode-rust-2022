#[macro_export]
macro_rules! run {
    ( $op:ident, $input:ident ) => {{
        let result = $op($input.lines());
        println!("{} {}: {}", module_path!(), std::stringify!($op), result);
    }};
}

#[macro_export]
macro_rules! verify {
    ( $op:ident, $input:ident, $expected:literal ) => {{
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
