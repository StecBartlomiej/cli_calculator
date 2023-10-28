use std::io::{self, Write};

pub mod calc;


fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut vector = Vec::new();

    const MAX_PAST_INPUT_SIZE: usize = 10;
    let mut past_input: [String; MAX_PAST_INPUT_SIZE] = Default::default();

    let mut iter: usize = 0;
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        if stdin.read_line(&mut line).is_err() {
            break;
        }

        past_input[iter % MAX_PAST_INPUT_SIZE] = line.clone();


        line.chars().filter(|x| *x != ' ' && *x != '\n').for_each(|x| vector.push(x as u8 as char));


        let mut tokens = calc::get_tokens(&vector);

        let result = calc::expression(&mut tokens);

        match result {
            Ok(v) => println!("{v}"),
            Err(e) => println!("Error: {e}")
        }

        line.clear();
        vector.clear();

        iter += 1;
    }
}
