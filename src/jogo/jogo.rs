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
    fn remove_baralho(&mut self) -> Result<Carta, String>{
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
    pub fn pull_cards(&mut self, index: i32) -> Result<String,String> {

        if index>=0 && (index as usize) < self.players.len(){
            
            let card = match self.remove_baralho() {
                Ok(c) => c,
                Err(e) => {
                    println!("Erro {}",e);
                    return Err(String::from("Erro ao pegar carta"));
                }
            };

            self.players[index as usize].add_card(card);

            Ok(format!("Player {} pegou uma carta", self.players[index as usize].get_name()))
        
        }else{
            return Err(String::from("indice de jogador maior que o numeri de players"));
        }

    } 
    pub fn down_3_cards_letter(&mut self,index: u32, cards: Vec<u32>) -> Result<[bool;5],String>{

        if (index as usize) < self.players.len(){
            if cards.iter().all(|card| *card < self.players[index as usize].get_cards().len() as u32) {

                let mut analise:Vec<Carta> = Vec::new();
                let mut num_de_dois: u32 = 0;
                let mut options_push: [bool;5] = [false,false,false,false,false];

                for i in cards {
                    match self.players[index as usize].get_card(i as u32) {
                        Ok(c) => analise.push(c),
                        Err(e) => {
                            println!("Erro: {}", e);
                            return Err(String::from("Erro ao pegar carta"));
                        }
                    }
                }

                analise.sort();

                if analise[0].get_carta().0 == analise[1].get_carta().0 && analise[2].get_carta().0 == analise[1].get_carta().0{
                    
                    if analise[1].get_carta().1 == analise[0].get_carta().1 && analise[2].get_carta().1 == analise[1].get_carta().1+1{
                        
                        print!("\nsequencial\n");
                        options_push[0]=true;
                    
                    }
                    if analise[0].get_carta().1 == 2 && 
                        (analise[2].get_carta().1 == analise[1].get_carta().1+2 ||
                         analise[2].get_carta().1 == analise[1].get_carta().1+1){

                        print!("\n2+sequencial\n");
                        options_push[1]=true;
                    
                    }
                    if (analise[1].get_carta().1 == analise[0].get_carta().1+2 ||
                        analise[1].get_carta().1 == analise[0].get_carta().1+1) && 
                        analise[2].get_carta().1 == 2{

                        print!("\nsequencial+2\n");
                        options_push[3]=true;

                    }
                    if analise[2].get_carta().1 == analise[0].get_carta().1+2 &&
                      analise[1].get_carta().1 == 2{
                        
                        print!("\ncarta+2+carta\n");
                        options_push[2]=true;
                        
                    }
                    
                    print!("\nmesmo nipe\n");
                    options_push[4]=true;
                    
                }else if analise[0].get_carta().0 == analise[1].get_carta().0 || analise[2].get_carta().0 == analise[1].get_carta().0{
                    
                    if analise[0].get_carta().1 == 2 &&
                       (analise[2].get_carta().1 == analise[1].get_carta().1+1 || 
                       analise[2].get_carta().1 == analise[1].get_carta().1+2) &&
                       analise[2].get_carta().0 == analise[1].get_carta().0{

                        print!("\n2+sequencial\n");
                        options_push[1]=true;
                    
                    }
                    if (analise[1].get_carta().1 == analise[0].get_carta().1+1 || 
                        analise[1].get_carta().1 == analise[0].get_carta().1+2) &&
                        analise[1].get_carta().0 == analise[0].get_carta().0 && 
                        analise[2].get_carta().1 == 2{

                        print!("\nsequencial+2\n");
                        options_push[3]=true;

                    }
                    if  analise[2].get_carta().1 == analise[0].get_carta().1+2 &&
                        analise[2].get_carta().0 == analise[0].get_carta().0 && 
                        analise[1].get_carta().1 == 2{
                        
                        print!("\ncarta+2+carta\n");
                        options_push[2]=true;

                    }
                 
                 print!("\nnipe diferente\n");
                }
                
                
                /* for i in 0..3{

                    if let Some(index) = self.players[index as usize].get_cards().iter().position(|x| 
                        x.get_carta().0 == analise[i as usize].get_carta().0 &&
                        x.get_carta().1 == analise[i as usize].get_carta().1 
                    ) {
                        println!("O valor {} está na posição {}", analise[i as usize], index);
                    } else {
                        println!("O valor {} não está no vetor", analise[i as usize]);
                    }

                    println!("{}",analise[i as usize]);
                } */

                println!("{:?}",options_push);
                
                if options_push[0] != true || options_push[1] != true || options_push[2] != true || options_push[3] != true || options_push[4] != true{
                    return Ok(options_push);
                }else {
                    return Err(String::from("Este grupo de cartas não podem ir para a mesa"));
                }
            }        
        }

        Err(String::from("As cartas escolhidas nao podem descer para mesa"))
    }
}