fn main() {
    // Data Types
    // **Scalar & Compound

    // Signed Variants can store numbers from -(2^(n-1)) to (2^(n-1)-1)
    // Unsigned Variants can store numbers from 0 to (2^n)-1cd
    let num : i8 = "-200".parse().expect("Not a number");

    println!("{num}");
}
