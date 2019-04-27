use rand::seq::SliceRandom;

pub struct Robot(String);

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER: &[u8] = b"0123456789";

fn id_generator() -> String {
    let mut rng = &mut rand::thread_rng();

    let alphabet = ALPHABET.choose_multiple(&mut rng, 2).cloned();
    let number = NUMBER.choose_multiple(&mut rng, 3).cloned();
    let id = alphabet.chain(number).collect();
    String::from_utf8(id).unwrap_or_default()
}

impl Robot {
    pub fn new() -> Self {
        Robot(id_generator())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = id_generator();
    }
}
