extern crate rand;

use crate::enums::nipe::Nipe;
use crate::enums::efeito::Efeito;

use rand::Rng;
use std::fmt;

#[derive(Clone,Debug)]
pub struct Carta {
    nipe: Nipe,
    numero: u32,
    efeito: Efeito
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
                "Espadas" => Nipe::Espadas(String::from("♠")),
                "Paus" => Nipe::Paus(String::from("♣")),
                "Copas" => Nipe::Copas(String::from("♥")),
                "Ouro" => Nipe::Ouro(String::from("♦")),
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
                Efeito::Comum(String::from("#98dafb"))
            }
        };

        Ok(card)
        
    }
    
    fn rand_efect() -> Efeito {

        let rng = rand::thread_rng().gen_range(0..100);

        if rng == 0 {// 1%
            return Efeito::Exotico(String::from("#f0ed88"));
        } else if rng >= 1 && rng <= 10 {// 10%
            return Efeito::Lendario(String::from("#f859ff"));
        } else if rng >= 11 && rng <=30 {// 20%
            return Efeito::Raro(String::from("#01c96d"));
        }else {// 69%
            return Efeito::Comum(String::from("#98dafb"));
        }
        
    }

    pub fn get_carta(&self) -> (Nipe, u32, Efeito) {
        (
            self.nipe.clone(),
            self.numero,
            self.efeito.clone()
        )
    }

    pub fn print_carta(&self){

        println!("\tCarta");
        println!("Nipe: {}", self.nipe);
        println!("Numero: {}", self.numero);
        println!("Efeito: {}", self.efeito);

    }
    
}