use std::fmt;
use std::clone::Clone;

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Efeito{
    Comum(String),
    Raro(String),
    Lendario(String),
    Exotico(String)
}

impl fmt::Display for Efeito {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Efeito::Comum(s) => write!(f, "{}", s),
            Efeito::Raro(s) => write!(f, "{}", s),
            Efeito::Lendario(s) => write!(f, "{}", s),
            Efeito::Exotico(s) => write!(f, "{}", s),
        }
    }
}
