use enigma_machine::{self, Setting};
use std::io::{self, Write};

fn main() {
    print!("Enter a message to encrypt: ");
    io::stdout().flush().expect("Failed to flush");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    let plugboard = enigma_machine::Plugboard::new(&vec![
        ('A', 'L'),
        ('P', 'R'),
        ('T', 'D'),
        ('B', 'W'),
        ('K', 'F'),
        ('O', 'Y'),
    ]);

    let mut setting = Setting::new(
        &plugboard,
        enigma_machine::rotor::I,
        enigma_machine::rotor::II,
        enigma_machine::rotor::III,
        'A',
        'G',
        'F',
        'A',
        'B',
        'D',
        enigma_machine::reflector::UKW_B,
    );

    message = enigma_machine::run_message(&message, &mut setting);
    println!("Output: {message}");
}
