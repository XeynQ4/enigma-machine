use crate::{Plugboard, Reflector, Rotor};

pub struct Setting {
    pub plugboard: Plugboard,
    pub rotor1: Rotor,
    pub rotor2: Rotor,
    pub rotor3: Rotor,
    pub rotor1_position: char,
    pub rotor2_position: char,
    pub rotor3_position: char,
    pub ring_setting1: char,
    pub ring_setting2: char,
    pub ring_setting3: char,
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
        ring_setting1: char,
        ring_setting2: char,
        ring_setting3: char,
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
            ring_setting1,
            ring_setting2,
            ring_setting3,
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
        ring_setting1: char,
        ring_setting2: char,
        ring_setting3: char,
        reflector: Reflector,
    ) {
        self.plugboard = plugboard.clone();
        self.rotor1 = rotor1;
        self.rotor2 = rotor2;
        self.rotor3 = rotor3;
        self.rotor1_position = rotor1_position;
        self.rotor2_position = rotor2_position;
        self.rotor3_position = rotor3_position;
        self.ring_setting1 = ring_setting1;
        self.ring_setting2 = ring_setting2;
        self.ring_setting3 = ring_setting3;
        self.reflector = reflector;
    }

    fn move_rotors(&mut self) {
        self.rotor3_position = (((self.rotor3_position as u8 - 65 + 1) % 26) + 65) as char;
        if self.rotor3_position == self.rotor3.turnover {
            self.rotor2_position = (((self.rotor2_position as u8 - 65 + 1) % 26) + 65) as char;
            if self.rotor2_position == self.rotor2.turnover {
                self.rotor1_position = (((self.rotor1_position as u8 - 65 + 1) % 26) + 65) as char;
            }
        } else if self.rotor2_position
            == (((self.rotor2.turnover as u8 - 65 + 26 - 1) % 26) + 65) as char
        {
            self.rotor2_position = self.rotor2.turnover;
            self.rotor1_position = (((self.rotor1_position as u8 - 65 + 1) % 26) + 65) as char;
        }
    }

    pub fn run(&mut self, letter: char) -> char {
        self.move_rotors();

        let mut letter = self.plugboard.run(letter);
        letter = self
            .rotor3
            .run_right_left(letter, self.rotor3_position, self.ring_setting3);
        letter = self
            .rotor2
            .run_right_left(letter, self.rotor2_position, self.ring_setting2);
        letter = self
            .rotor1
            .run_right_left(letter, self.rotor1_position, self.ring_setting1);
        letter = self.reflector.run(letter);
        letter = self
            .rotor1
            .run_left_right(letter, self.rotor1_position, self.ring_setting1);
        letter = self
            .rotor2
            .run_left_right(letter, self.rotor2_position, self.ring_setting2);
        letter = self
            .rotor3
            .run_left_right(letter, self.rotor3_position, self.ring_setting3);
        self.plugboard.run(letter)
    }
}
