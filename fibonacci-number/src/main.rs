use std::io;

fn main() {
    
    let mut number = String::new();
    
    println!("Please the 'n' of fibonacci-number:");
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number:usize = number.trim().parse().expect("You must enter a number");

    println!("You entered: {}",number);
    
    let mut fibonacci_number: Vec<usize> = Vec::new();
    
    if number == 0 {
        fibonacci_number.push(0);
    } else if number == 1 {
        fibonacci_number.push(0);
        fibonacci_number.push(1);
    } else {
        fibonacci_number.push(0);
        fibonacci_number.push(1);
        
        let mut index = 2;
        while index <= number {
            let number_index = &fibonacci_number[index - 1] + &fibonacci_number[index - 2];
            fibonacci_number.push(number_index); 
            index = index + 1 ;
        }
    }

    for element in fibonacci_number.iter() {
            print!("{} ",element);
    }
    println!("");

}
