use requestty::{Answers, Question};

fn main() {
    let password = Question::password("password")
        .message("What is your password?")
        .mask('*')
        .validate(password_validator)
        .build();
    let password_confirm = Question::password("password-confirm")
        .message("Please retype your password.")
        .mask('*')
        .validate(confirm_password_validator)
        .build();

    let questions = vec![password, password_confirm];

    println!("{:#?}", requestty::prompt(questions));
}

fn password_validator(password: &str, _: &Answers) -> Result<(), String> {
    if password.len() < 5 {
        Err("Must be at least 6 characters.".into())
    } else if !password.contains(|c: char| c.is_ascii_digit()) {
        Err("Needs at least one number.".into())
    } else if !password.contains(char::is_alphabetic) {
        Err("Needs at least one letter of the alphabet.".into())
    } else {
        Ok(())
    }
}

fn confirm_password_validator(password_confirm: &str, answers: &Answers) -> Result<(), String> {
    let base_password = answers.get("password").unwrap().as_string().unwrap();

    if password_confirm.eq(base_password) {
        Ok(())
    } else {
        Err("Does not match your password.".into())
    }
}
