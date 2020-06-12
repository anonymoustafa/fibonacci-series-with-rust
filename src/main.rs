use std::io;

fn main() {
    println!("Input the fibonacci index you want me to ask for calculation ");
    let mut fibonacci_index = String::new();
    io::stdin()
        .read_line(&mut fibonacci_index)
        .expect("Failed to read line");
    let fibonacci_index: u32 = return match fibonacci_index.trim().parse() {
        Ok(_) => {},
        Err(_) => {},
    };
    for i in 1..fibonacci_index {
        if fibonacci_index <= 0 {println!("error, index must be zero or bigger.");}
        else if fibonacci_index == 0 || fibonacci_index == 1 {println!("1");}
        else { println!("{}", fibonacci_series);}
    }
}

fn fibonacci_series(fibonacci_index: us32) -> usize {
    let mut index = 0;

}