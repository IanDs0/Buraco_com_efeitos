use std::fmt;
use std::clone::Clone;
use std::cmp::Ordering;

#[derive(Clone,Copy,Debug,Eq)]
pub enum Nipe {
    Espadas(&'static str),
    Paus(&'static str),
    Copas(&'static str),
    Ouro(&'static str)
}

impl PartialOrd for Nipe {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Nipe {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
    
impl Ord for Nipe {
    fn cmp(&self, other: &Self) -> Ordering {
        // Adicione a lógica de comparação que você deseja para as instâncias de Nipe
        match (self, other) {
            (Nipe::Espadas("♠"), Nipe::Paus("♣")) => Ordering::Less,
            (Nipe::Espadas("♠"), Nipe::Copas("♥")) => Ordering::Less,
            (Nipe::Espadas("♠"), Nipe::Ouro("♦")) => Ordering::Less,

            (Nipe::Paus("♣"), Nipe::Copas("♥")) => Ordering::Less,
            (Nipe::Paus("♣"), Nipe::Ouro("♦")) => Ordering::Less,

            (Nipe::Copas("♥"), Nipe::Ouro("♦")) => Ordering::Less,
            
            (Nipe::Espadas("♠"), Nipe::Espadas("♠")) => Ordering::Equal,
            (Nipe::Paus("♣"), Nipe::Paus("♣")) => Ordering::Equal,
            (Nipe::Copas("♥"), Nipe::Copas("♥")) => Ordering::Equal,
            (Nipe::Ouro("♦"), Nipe::Ouro("♦")) => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
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