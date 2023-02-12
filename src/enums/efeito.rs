use std::fmt;
use std::clone::Clone;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Efeito {
    Comum(&'static str),
    Raro(&'static str),
    Lendario(&'static str),
    Exotico(&'static str),
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