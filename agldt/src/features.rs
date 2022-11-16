use std::error::Error;
use std::fmt::Display;

pub struct POSFeature {
    index: u8,
    char: char,
}

impl POSFeature {
    pub fn new(index: u8, char: char) -> Result<Self, Box<dyn Error>> {
        if index > 9 {
            panic!("Index out of bounds, max = 8");
        } else {
            Ok(Self { index, char })
        }
    }
    pub fn index(&self) -> u8 {
        self.index
    }
    pub fn char(&self) -> char {
        self.char
    }
}

impl Display for POSFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

pub trait PostagFeature {
    fn to_agldt_postag(&self) -> POSFeature;
    fn to_string(&self) -> String;
}
