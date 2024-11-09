/// A struct representing the rotors of the Enigma machine.
#[derive(Clone, Copy)]
pub struct Rotor {
    wiring: [char; 26],
    /// The letter at which the rotor turns over the one to its left.
    pub turnover: char,
}

impl Rotor {
    /// Runs the given letter through the rotor from right to left.
    pub fn run_right_left(&self, letter: char, position: char, ring: char) -> char {
        let letter = (letter as u8 + position as u8 - ring as u8 + 26 - 65) % 26;
        (((self.wiring[letter as usize] as u8 + 26 - position as u8 + ring as u8 - 65) % 26) + 65)
            as char
    }

    /// Runs the given letter through the rotor from left to right.
    pub fn run_left_right(&self, letter: char, position: char, ring: char) -> char {
        let letter = (((letter as u8 - 65 + position as u8 + 26 - ring as u8) % 26) + 65) as char;
        (((self.wiring.iter().position(|&x| x == letter).unwrap() as u8 + 26 + ring as u8
            - position as u8)
            % 26)
            + 65) as char
    }
}

/// The rotor I used in the real Enigma machine.
pub static I: Rotor = Rotor {
    wiring: [
        'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U',
        'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J',
    ],
    turnover: 'R',
};

/// The rotor II used in the real Enigma machine.
pub static II: Rotor = Rotor {
    wiring: [
        'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G',
        'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E',
    ],
    turnover: 'F',
};

/// The rotor III used in the real Enigma machine.
pub static III: Rotor = Rotor {
    wiring: [
        'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W',
        'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
    ],
    turnover: 'W',
};

/// The rotor IV used in the real Enigma machine.
pub static IV: Rotor = Rotor {
    wiring: [
        'E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F',
        'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B',
    ],
    turnover: 'K',
};

/// The rotor V used in the real Enigma machine.
pub static V: Rotor = Rotor {
    wiring: [
        'V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W',
        'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K',
    ],
    turnover: 'A',
};
