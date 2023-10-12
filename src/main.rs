use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn choose() {
    println!("Hello, world!");
    
    let my_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess my number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a number...");
                    continue;
                },
            };

        match guess.cmp(&my_number) {
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => {
                println!("Bingo!");
                break;
            }
        }
    }
}


fn main() {
    let ans: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = ans;
    let a = ans.0;
    let arr_a = [1,2,3];
    let arr_b = [3; 5];
    println!("{:?}", arr_a);
    println!("{:?}", arr_b);
}
