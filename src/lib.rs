mod plugboard;
pub mod reflector;
pub mod rotor;
mod setting;
pub use plugboard::Plugboard;
pub use reflector::Reflector;
pub use rotor::Rotor;
pub use setting::Setting;

pub fn run_message(message: &str, setting: &mut Setting) -> String {
    let mut message = message.to_string();
    message = clean_up_message(&message);

    let mut new_message = String::new();
    for letter in message.chars() {
        new_message.push(run_letter_without_cleanup(letter, setting));
    }
    new_message
}

fn run_letter_without_cleanup(letter: char, setting: &mut Setting) -> char {
    setting.run(letter)
}

pub fn run_letter(letter: char, setting: &mut Setting) -> char {
    run_letter_without_cleanup(clean_up_letter(letter), setting)
}

fn clean_up_letter(letter: char) -> char {
    if !letter.is_ascii_alphabetic() {
        panic!(
            "Invalid letter: {}. Letters have to be ASCII alphabetic characters",
            letter
        );
    }
    letter.to_ascii_uppercase()
}

fn clean_up_message(message: &str) -> String {
    let mut message = message.to_uppercase();
    message.retain(|c| c.is_ascii_alphabetic());
    message
}