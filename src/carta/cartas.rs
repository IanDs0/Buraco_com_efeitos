extern crate rand;

use crate::enums::nipe::Nipe;
use crate::enums::efeito::Efeito;
use std::cmp::Ordering;

use rand::Rng;
use std::fmt;

#[derive(Clone,Copy,Debug,Eq)]
pub struct Carta {
    nipe: Nipe,
    numero: u32,
    efeito: Efeito
}

impl PartialEq for Carta {
    fn eq(&self, other: &Self) -> bool {
        self.nipe == other.nipe && self.numero == other.numero
    }
}

impl Ord for Carta {
    fn cmp(&self, other: &Self) -> Ordering {
        let nipe_result = self.nipe.cmp(&other.nipe);
        let numero_result = self.numero.cmp(&other.numero);
        match (nipe_result, numero_result) {
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Equal, numero_result) => numero_result,
            (nipe_result, _) => nipe_result,
        } 
    }
}
impl PartialOrd for Carta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Carta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Carta\nNipe: {}\nNumero: {}\nEfeito: {}", self.nipe, self.numero, self.efeito)
    }
}

impl Carta {

    pub fn new(nipe: &String, numero: u32, efeito: bool) -> Result<Carta, String> {
        
        let card = Carta {
            nipe: match nipe.as_str(){
                "Espadas" => Nipe::Espadas("♠"),
                "Paus" => Nipe::Paus("♣"),
                "Copas" => Nipe::Copas("♥"),
                "Ouro" => Nipe::Ouro("♦"),
                _ => return Err(String::from("nipe invalido")),
            },
            
            numero: if numero > 0 && numero < 14 { 
                numero
            } else { 
                return Err(String::from("Numero de carta invalido"));
            },

            efeito: if efeito { 
                Self::rand_efect()
            } else { 
                Efeito::Comum("#98dafb")
            }
        };

        Ok(card)
        
    }
    
    fn rand_efect() -> Efeito {

        let rng = rand::thread_rng().gen_range(0..100);

        if rng == 0 {// 1%
            return Efeito::Exotico("#f0ed88");
        } else if rng >= 1 && rng <= 10 {// 10%
            return Efeito::Lendario("#f859ff");
        } else if rng >= 11 && rng <=30 {// 20%
            return Efeito::Raro("#01c96d");
        }else {// 69%
            return Efeito::Comum("#98dafb");
        }
        
    }

    pub fn get_carta(&self) -> (Nipe, u32, Efeito) {
        (
            self.nipe.clone(),
            self.numero,
            self.efeito.clone()
        )
    }
    
}