/// A struct representing the reflectors of the Enigma machine.
#[derive(Clone, Copy)]
pub struct Reflector {
    wiring: [char; 26],
}

impl Reflector {
    /// Runs the given letter through the reflector.
    pub fn run(&self, letter: char) -> char {
        self.wiring[(letter as u8 - 65) as usize]
    }
}

/// The reflector UKW-A used in the real Enigma machine.
pub static UKW_A: Reflector = Reflector {
    wiring: [
        'E', 'J', 'M', 'Z', 'A', 'L', 'Y', 'X', 'V', 'B', 'W', 'F', 'C', 'R', 'Q', 'U', 'O', 'N',
        'T', 'S', 'P', 'I', 'K', 'H', 'G', 'D',
    ],
};

/// The reflector UKW-B used in the real Enigma machine.
pub static UKW_B: Reflector = Reflector {
    wiring: [
        'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B',
        'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T',
    ],
};

/// The reflector UKW-C used in the real Enigma machine.
pub static UKW_C: Reflector = Reflector {
    wiring: [
        'F', 'V', 'P', 'J', 'I', 'A', 'O', 'Y', 'E', 'D', 'R', 'Z', 'X', 'W', 'G', 'C', 'T', 'K',
        'U', 'Q', 'S', 'B', 'N', 'M', 'H', 'L',
    ],
};
