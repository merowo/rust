use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        
        let input = input.trim();
        if input == "exit" || input == "quit" || input == "q" {
            break;
        }
        else{
            println!("You inputted: {}", input);
    }
}
}
