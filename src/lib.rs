pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

#[macro_export]
macro_rules! input_ext {
    ($name:literal, $ext:expr) => {{
        let u: usize = $name;
        let file = format!("./inputs/day{:0>2}{}.txt", u, $ext);
        std::fs::read_to_string(file).expect("could not read file")
    }};
}

#[macro_export]
macro_rules! input {
    ($name:literal) => {{
        $crate::input_ext!($name, "")
    }};
}

#[macro_export]
macro_rules! input_example {
    ($name:literal, $num:literal) => {{
        let a: &'static str = concat!("_example", $num);
        $crate::input_ext!($name, a)
    }};
}
