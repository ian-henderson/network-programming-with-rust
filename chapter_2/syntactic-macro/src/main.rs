use std::env;

macro_rules! factorial {
    ($x:expr) => {{
        let mut result = 1;
        for i in 1..($x + 1) {
            result *= i;
        }

        result
    }};
}

fn main() {
    let arg = env::args()
        .nth(1)
        .expect("Please provide only one argument");

    println!(
        "{:?}",
        factorial!(arg.parse::<u64>().expect("Could not parse to an integer"))
    );
}
