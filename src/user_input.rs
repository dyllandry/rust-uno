use std::io;

pub fn get_user_input() -> Option<Input> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let input_without_newline = user_input.trim_end();
    match input_without_newline.parse::<i32>() {
        Ok(number) => Some(Input::Number(number)),
        Err(_) => Some(Input::Text(input_without_newline.to_string())),
    }
}

#[derive(std::fmt::Debug)]
pub enum Input {
    Text(String),
    Number(i32),
}
