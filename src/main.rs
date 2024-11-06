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
        // ('A', 'L'),
        // ('P', 'R'),
        // ('T', 'D'),
        // ('B', 'W'),
        // ('K', 'F'),
        // ('O', 'Y'),
        // ('A', 'B'),
        // ('C', 'D'),
        // ('E', 'F'),
        // ('G', 'H'),
        // ('I', 'J'),
        // ('K', 'L'),
        // ('M', 'N'),
        // ('O', 'P'),
        // ('Q', 'R'),
        // ('S', 'T'),
        // ('U', 'V'),
        // ('W', 'X'),
        // ('Y', 'Z'),
    ]);

    let mut setting = Setting::new(
        &plugboard,
        enigma_machine::rotor::I,
        enigma_machine::rotor::II,
        enigma_machine::rotor::III,
        'P',
        'D',
        'T',
        enigma_machine::reflector::UKW_B,
    );

    // let letter = enigma_machine::run_letter('A', &setting);
    // println!("Letter after plugboard: {letter}");

    message = enigma_machine::run_message(&message, &mut setting);
    println!("Output: {message}");

    // message = enigma_machine::run_message(&message, &mut setting);
    // println!("Encoded message: {message}");
}
