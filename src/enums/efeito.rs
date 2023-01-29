use std::fmt;
use std::clone::Clone;

#[derive(Clone,Debug)]
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
