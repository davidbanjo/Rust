use std::io;

fn main() {
    println!("Guessing Game! Enter any Number");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    println!("#######Simple Calculator###########");

    let mut first_num = String::new();
    let mut second_num = String::new();

    println!("Enter first number ");
    io::stdin()
        .read_line(&mut first_num)
        .expect("Failed to read input");

    println!("Enter second number ");
    io::stdin()
        .read_line(&mut second_num)
        .expect("Failed to read input");

    println!("The sum is : {}", first_num + &second_num);

}