use crate::{Plugboard, Reflector, Rotor};

/// A struct representing the settings of an Enigma machine.
pub struct Setting {
    /// The plugboard configuration used, represented as the `Plugboard` struct.
    pub plugboard: Plugboard,
    /// The rotor in the leftmost slot, represented as the `Rotor` struct.
    pub rotor1: Rotor,
    /// The rotor in the middle slot, represented as the `Rotor` struct.
    pub rotor2: Rotor,
    /// The rotor in the rightmost slot, represented as the `Rotor` struct.
    pub rotor3: Rotor,
    /// The current position of the leftmost rotor.
    pub rotor1_position: char,
    /// The current position of the middle rotor.
    pub rotor2_position: char,
    /// The current position of the rightmost rotor.
    pub rotor3_position: char,
    /// The ring setting of the leftmost rotor.
    pub ring_setting1: char,
    /// The ring setting of the middle rotor.
    pub ring_setting2: char,
    /// The ring setting of the rightmost rotor.
    pub ring_setting3: char,
    /// The reflector used, represented as the `Reflector` struct.
    pub reflector: Reflector,
}

impl Setting {
    /// Creates a new `Setting` struct with the given parameters.
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

    /// Sets the parameters of the `Setting` struct to the given values.
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

    /// Runs the given letter through the Enigma machine.
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
