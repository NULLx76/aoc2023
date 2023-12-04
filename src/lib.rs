pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

#[macro_export]
macro_rules! input_ext {
    ($name:expr, $ext:expr) => {{
        let u: usize = $name;
        let file = format!("./inputs/day{:0>2}{}.txt", u, $ext);
        std::fs::read_to_string(file).unwrap_or(String::from("file not found"))
    }};
}

#[macro_export]
macro_rules! input {
    ($name:expr) => {{
        $crate::input_ext!($name, "")
    }};
}

#[macro_export]
macro_rules! input_example {
    ($name:expr, $num:literal) => {{
        let a: &'static str = concat!("_example", $num);
        $crate::input_ext!($name, a)
    }};
}
