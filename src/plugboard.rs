/// A struct representing the plugboard of the Enigma machine.
#[derive(Clone)]
pub struct Plugboard {
    connections: Vec<(char, char)>,
}

impl Plugboard {
    /// Creates a new plugboard with the given connections.
    pub fn new(connections: &Vec<(char, char)>) -> Self {
        if !Self::are_valid_connections(connections) {
            panic!("Invalid plugboard connections");
        }

        Self {
            connections: connections.clone(),
        }
    }

    /// Sets the connections of the plugboard.
    pub fn set(&mut self, connections: &Vec<(char, char)>) {
        if !Self::are_valid_connections(connections) {
            panic!("Invalid plugboard connections");
        }

        self.connections = connections.clone();
    }

    fn are_valid_connections(connections: &Vec<(char, char)>) -> bool {
        let mut used_letters = Vec::new();

        for (letter1, letter2) in connections {
            if used_letters.contains(letter1) || used_letters.contains(letter2) {
                return false;
            }

            used_letters.push(*letter1);
            used_letters.push(*letter2);
        }

        true
    }

    /// Runs the given letter through the plugboard.
    pub fn run(&self, letter: char) -> char {
        for (letter1, letter2) in &self.connections {
            if letter == *letter1 {
                return *letter2;
            } else if letter == *letter2 {
                return *letter1;
            }
        }

        letter
    }

    /// Returns the connections of the plugboard.
    pub fn connections(&self) -> &Vec<(char, char)> {
        &self.connections
    }
}
