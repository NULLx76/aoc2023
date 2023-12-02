pub mod day01;
pub mod day02;

#[macro_export]
macro_rules! input {
    ($name:literal) => {{
        let u: usize = $name;
        let file = format!("./inputs/day{:0>2}.txt", u);
        std::fs::read_to_string(file).expect("could not read file")
    }};
}

#[macro_export]
macro_rules! input_example {
    ($name:literal, $num:literal) => {{
        let u: usize = $name;
        let n: usize = $num;
        let file = format!("./inputs/day{:0>2}_example{}.txt", u, n);
        std::fs::read_to_string(file).expect("could not read file")
    }};
}
