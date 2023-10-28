use std::io::{self, Write};


pub mod calc;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut vector = Vec::new();

    loop {
        print!(">");
        io::stdout().flush().unwrap();

        if stdin.read_line(&mut line).is_err() {
            break;
        }
        line.chars().filter(|x| *x != ' ' && *x != '\n').for_each(|x| vector.push(x as u8 as char));

        let mut tokens = calc::get_tokens(&vector);

        println!("{}", calc::expression(&mut tokens));

        line.clear();
        vector.clear();
    }
}
