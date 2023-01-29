extern crate rand;

use crate::carta::cartas::Carta;
use crate::user::users::User; 

use std::collections::VecDeque;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone,Copy,Debug)]
pub struct Jogo {
    players: Vec<User>,
    baralho: VecDeque<Carta>,
}

impl Jogo {

    pub fn new() -> Jogo {
       Jogo {
           players: Vec::new(),
           baralho: VecDeque::new(),
       } 
    }

    //Baralho
    pub fn set_baralho(&mut self){
        let nipes: Vec<String> = vec![
            "Espadas".to_string(),
            "Paus".to_string(),
            "Copas".to_string(),
            "Ouro".to_string()
        ];
        let mut cartas_Baralho = Vec::new();
        for (nipe, numero) in nipes.iter().cartesian_product(1..13) {
            cartas_Baralho.push(
                match Carta::new(nipe, numero, true) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Erro: {}", e);
                        continue;
                    }
                }
            )
        };
        
        cartas_Baralho.shuffle(&mut thread_rng());
        
        self.baralho.extend(cartas_Baralho);
    }
    pub fn get_baralho(&self) -> VecDeque<Carta>{
        self.baralho
    }
    pub fn remove_baralho(&mut self) -> Result<Carta, String>{
        match self.baralho.pop_back(){
                Some(c) => return Ok(c),
                None => return Err(String::from("Baralho vazio"))
        } 
    }

    pub fn set_user(&mut self ,name: String, points: i32, hp: u16) -> Result<Self,String> {

        let mut user = User::new();

        let mut cards:Vec<Carta> = Vec::new();

        cards.push( match self.remove_baralho() {
            Ok(c) => c,
            Err(e) => {
                println!("Erro: {}", e);
                return Err(String::from("Erro ao entregar cartas"));
            }
        });

        user.set_user(
            name, 
            points, 
            hp, 
            cards.clone()
        );

        self.players.push(user);

        Ok(Jogo {
            players: ,
            baralho: self.get_baralho()
        })
    }

}