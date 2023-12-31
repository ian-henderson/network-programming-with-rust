trait Max<T> {
    fn max(&self) -> T;
}

struct ThreeTuple<T> {
    first: T,
    second: T,
    third: T,
}

// PartialOrd enables comparing
impl<T: PartialOrd + Copy> Max<T> for ThreeTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second && self.first >= self.third {
            self.first
        } else if self.second >= self.first && self.second >= self.third {
            self.second
        } else {
            self.third
        }
    }
}

struct TwoTuple<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Copy> Max<T> for TwoTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let two_tuple = TwoTuple { first: 4u32, second: 2u32 };
    let three_tuple = ThreeTuple { first: 4u64, second: 2u64, third: 10u64 };

    println!("{}", two_tuple.max());
    println!("{}", three_tuple.max());
}

