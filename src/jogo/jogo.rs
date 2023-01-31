extern crate rand;

use crate::carta::cartas::Carta;
use crate::user::users::User; 

use std::collections::VecDeque;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone,Debug)]
pub struct Jogo {
    pub players: Vec<User>,
    baralho: VecDeque<Carta>,
}

impl Jogo {

    pub fn new() -> Jogo {
       let mut jogo = Jogo {
           players: Vec::new(),
           baralho: VecDeque::new(),
       };
       jogo.set_baralho();
       jogo 
    }

    //Baralho
    fn set_baralho(&mut self){
        let nipes: Vec<String> = vec![
            "Espadas".to_string(),
            "Paus".to_string(),
            "Copas".to_string(),
            "Ouro".to_string()
        ];
        let mut cartas_baralho = Vec::new();
        for _ in 0..2 {
            for (nipe, numero) in nipes.iter().cartesian_product(1..13) {
                cartas_baralho.push(
                    match Carta::new(nipe, numero, true) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Erro: {}", e);
                            continue;
                        }
                    }
                )
            }
        };
        
        cartas_baralho.shuffle(&mut thread_rng());
        
        self.baralho.extend(cartas_baralho);
    }
    fn get_baralho(&self) -> VecDeque<Carta>{
        self.baralho.clone()
    }
    pub fn remove_baralho(&mut self) -> Result<Carta, String>{
        match self.baralho.pop_back(){
                Some(c) => return Ok(c),
                None => return Err(String::from("Baralho vazio"))
        } 
    }

    //Jogador
    pub fn set_user(&mut self ,name: String, points: i32, hp: u16) -> Result<Self,String> {

        let mut user = User::new();

        let mut cards:Vec<Carta> = Vec::new();
        
        let mut lixo:Vec<Carta> = Vec::new();

        for _ in 0..11 {
            cards.push( match self.remove_baralho() {
                Ok(c) => c,
                Err(e) => {
                    println!("Erro: {}", e);
                    return Err(String::from("Erro ao entregar cartas"));
                }
            });
        }

        for _ in 0..11 {
            lixo.push( match self.remove_baralho() {
                Ok(c) => c,
                Err(e) => {
                    println!("Erro: {}", e);
                    return Err(String::from("Erro ao entregar cartas"));
                }
            });
        }
        

        user.set_user(
            name, 
            points, 
            hp, 
            cards.clone(),
            lixo.clone(),
        );

        self.players.push(user);

        Ok(Jogo {
            players: self.get_user(),
            baralho: self.get_baralho()
        })
    }

    fn get_user(&self) -> Vec<User> {
        self.players.clone()
    }

}