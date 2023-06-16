fn main() {
    //VARIABLES AND MUTABILITY**************
    let x = 5; // immutable
    println!("The value of x is : {x}");

    let mut y = 6;
    println!("The value of y is : {y}");
    y = 7;
    println!("The value of y is : {y}");

    //CONSTANTS************
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const ONE: i32 = 1;
    const TWO: i32 = 2;

    const THREE: i32 = ONE + TWO;
    println!("The sum of constants isu {THREE}");

    //SHADOWING**************
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is : {x}");
    }
    println!("The value of y is : {x}");

    let spaces: &str = "david";
    let spaces:usize = spaces.len();
    println!("The number of spaces is {spaces}");
}
