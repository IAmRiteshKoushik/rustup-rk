use std::fmt;

// If you are not using Debug mode then you must implement a dispaly function
// which is compatible and allow a non-primitive ot print out its value
struct Structure(i32);

// Ro use the {} marker, the trait fmt::Display must be implemented manually
// for a type
impl fmt::Display for Structure {
    // Thsi train requires fmt with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicats whetherthe
        // operation succeesed or failed. Noe that `write!` uses syntax which
        // is similar to `println!`.
        write!(f, "{}", self.0);
    }
}

// fmt::Display is not implemented for any of the generic types as we cannot
// assume the standard way of printing them. fmt::Debug is however implemented
// on everything

// -- Activity
#[derive(Debug)]
struct MinMax(i64, i64); // Anonymous struct = unnamed fields

// Implement `Dispaly` and `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point
        write(f, "({}, {})", self.0, self.1) // accessing values from anoym struct
    }
}

// Define a structure where the fields are nameable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y);
    }
}

fn main() {
    let minmax = MinMax(0, 14);
}
