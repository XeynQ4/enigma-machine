use crate::{Plugboard, Reflector, Rotor};

pub struct Setting {
    pub plugboard: Plugboard,
    pub rotor1: Rotor,
    pub rotor2: Rotor,
    pub rotor3: Rotor,
    pub rotor1_position: char,
    pub rotor2_position: char,
    pub rotor3_position: char,
    pub reflector: Reflector,
}

impl Setting {
    pub fn new(
        plugboard: &Plugboard,
        rotor1: Rotor,
        rotor2: Rotor,
        rotor3: Rotor,
        rotor1_position: char,
        rotor2_position: char,
        rotor3_position: char,
        reflector: Reflector,
    ) -> Self {
        Self {
            plugboard: plugboard.clone(),
            rotor1,
            rotor2,
            rotor3,
            rotor1_position,
            rotor2_position,
            rotor3_position,
            reflector,
        }
    }

    pub fn set(
        &mut self,
        plugboard: &Plugboard,
        rotor1: Rotor,
        rotor2: Rotor,
        rotor3: Rotor,
        rotor1_position: char,
        rotor2_position: char,
        rotor3_position: char,
        reflector: Reflector,
    ) {
        self.plugboard = plugboard.clone();
        self.rotor1 = rotor1;
        self.rotor2 = rotor2;
        self.rotor3 = rotor3;
        self.rotor1_position = rotor1_position;
        self.rotor2_position = rotor2_position;
        self.rotor3_position = rotor3_position;
        self.reflector = reflector;
    }

    fn move_rotors(&mut self) {
        self.rotor3_position = (((self.rotor3_position as u8 - 65 + 1) % 26) + 65) as char;
        if self.rotor3_position == self.rotor3.turnover {
            self.rotor2_position = (((self.rotor2_position as u8 - 65 + 1) % 26) + 65) as char;
            if self.rotor2_position == self.rotor2.turnover {
                self.rotor1_position = (((self.rotor1_position as u8 - 65 + 1) % 26) + 65) as char;
            }
        } else if self.rotor3_position
            == (((self.rotor3.turnover as u8 - 65 + 1) % 26) + 65) as char
        {
            if self.rotor2_position
                == (((self.rotor2.turnover as u8 - 65 + 26 - 1) % 26) + 65) as char
            {
                self.rotor2_position = self.rotor2.turnover;
                self.rotor1_position = (((self.rotor1_position as u8 - 65 + 1) % 26) + 65) as char;
            }
        }
    }

    pub fn run(&mut self, letter: char) -> char {
        println!("\nKeyboard Input: {}", letter);
        self.move_rotors();

        println!(
            "Rotor Positions: {}{}{}",
            self.rotor1_position, self.rotor2_position, self.rotor3_position
        );

        let mut letter = self.plugboard.run(letter);
        println!("Plugboard Output: {}", letter);

        letter = self.rotor3.run_right_left(letter, self.rotor3_position);
        println!("Rotor 3 Output: {}", letter);

        letter = self.rotor2.run_right_left(letter, self.rotor2_position);
        println!("Rotor 2 Output: {}", letter);

        letter = self.rotor1.run_right_left(letter, self.rotor1_position);
        println!("Rotor 1 Output: {}", letter);

        letter = self.reflector.run(letter);
        println!("Reflector Output: {}", letter);

        letter = self.rotor1.run_left_right(letter, self.rotor1_position);
        println!("Rotor 1 Output: {}", letter);

        letter = self.rotor2.run_left_right(letter, self.rotor2_position);
        println!("Rotor 2 Output: {}", letter);

        letter = self.rotor3.run_left_right(letter, self.rotor3_position);
        println!("Rotor 3 Output: {}", letter);

        letter = self.plugboard.run(letter);
        println!("Plugboard Output: {}", letter);

        letter
    }
}
