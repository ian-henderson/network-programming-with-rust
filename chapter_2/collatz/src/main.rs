// https://en.wikipedia.org/wiki/Collatz_conjecture

// This struct holds state while iterating
struct Collatz {
    current: u64,
    end: u64,
}

// Iterator implementation
impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current = if self.current % 2 == 0 {
            self.current / 2
        } else {
            3 * self.current + 1
        };

        if self.current == self.end {
            None
        } else {
            Some(self.current)
        }
    }
}

// utility function to start iteration
fn collatz(start: u64) -> Collatz {
    Collatz {
        current: start,
        end: 1u64,
    }
}

fn main() {
    let input = 10;

    // first two items
    for n in collatz(input).take(2) {
        println!("{}", n);
    }

    // dropping first two items
    for n in collatz(input).skip(2) {
        println!("{}", n);
    }
}
