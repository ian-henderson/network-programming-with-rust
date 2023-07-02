use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
enum OperationsError {
    DivideByZeroError,
}

// Useful for displaying the error nicely
impl Display for OperationsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            OperationsError::DivideByZeroError => f.write_str("Cannot divide by zero"),
        }
    }
}

// Registers the custom error as an error
impl Error for OperationsError {
    fn description(&self) -> &str {
        match *self {
            OperationsError::DivideByZeroError => "Cannot divide by zero",
        }
    }
}

// Divides the dividend by the divisor and returns the result.
// Returns an error if the divisor is zero.
fn divide(dividend: u32, divisor: u32) -> Result<u32, OperationsError> {
    if divisor == 0u32 {
        return Err(OperationsError::DivideByZeroError);
    }

    Ok(dividend / divisor)
}

fn main() {
    let result1 = divide(100, 0);
    println!("{:?}", result1);

    let result2 = divide(100, 2);
    println!("{:?}", result2.unwrap());
}
