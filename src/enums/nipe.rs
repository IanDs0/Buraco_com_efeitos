use std::fmt;
use std::clone::Clone;

#[derive(Clone,Debug)]
pub enum Nipe {
    Espadas(String),
    Paus(String),
    Copas(String),
    Ouro(String)
}

impl fmt::Display for Nipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nipe::Espadas(s) => write!(f, "{}", s),
            Nipe::Paus(s) => write!(f, "{}", s),
            Nipe::Copas(s) => write!(f, "{}", s),
            Nipe::Ouro(s) => write!(f, "{}", s),
        }
    }
}