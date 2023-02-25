use crate::carta::cartas::Carta;

use std::vec::Vec;

#[derive(Clone,Debug)]
pub struct User{
    name: String,
    points: i32,
    hp: u16,
    cards: Vec<Carta>,
    lixo: Vec<Carta>,
    pub mesa: Vec<Vec<Carta>>
}

impl User{
    pub fn new() -> User {
        User{
            name: String::new(),
            points: 0,
            hp: 0,
            cards: Vec::new(),
            lixo: Vec::new(),
            mesa: Vec::new(),
        }
    }
    
    //User
    pub fn set_user(&mut self ,name: String, points: i32, hp: u16, cards: Vec<Carta>, lixo: Vec<Carta>) {
        Self::set_name(self, name.to_string());
        Self::set_points(self, points);
        Self::set_hp(self, hp);
        Self::set_card(self, cards);
        Self::set_lixo(self, lixo);
    }
    pub fn get_user(&self) -> (String, i32, u16, Vec<Carta>, Vec<Carta>) {
        (
            self.get_name(),
            self.get_points(),
            self.get_hp(),
            self.get_cards(),
            self.get_lixo()
        )
    }
    pub fn print_user(&self){
        println!("Name: {}", self.name);
        println!("Points: {}", self.points);
        println!("HP: {}", self.hp);
        println!("Cards:");
        self.print_card();
    }

    //Points
    pub fn add_points(&mut self, add_points: i32) {
        self.points = self.points + add_points;
    }
    fn set_points(&mut self, points: i32) {
        self.points = points;
    }
    pub fn get_points(&self) -> i32 { 
        self.points
    }

    //HP
    pub fn health_hp(&mut self, health: u16) {
        self.hp = self.hp + health;
    }   
    pub fn damage_hp(&mut self, damage: u16) {
        self.hp = std::cmp::max(
            self.hp as i16 - damage as i16,
            0
        )as u16;
    }
    fn set_hp(&mut self, hp: u16) {
        self.hp = hp;
    }
    pub fn get_hp(&self) -> u16 {
        self.hp
    }

    //Lixo
    fn set_lixo(&mut self, card: Vec<Carta>) {
        self.lixo = card;
    }
    fn get_lixo(&self) -> Vec<Carta>{
        self.lixo.clone()
    }
    pub fn pegar_lixo(&mut self) -> Result<String,String>{
        if self.lixo.len() > 0{
            self.cards.extend(Self::get_lixo(&self));
            self.lixo.clear();
            Ok(String::from("O lixo voi pego"))
        }else {
            Err(String::from("O lixo ja foi pego"))
        }
    }

    //Cards
    pub fn add_card(&mut self, card: Carta){
        self.cards.push(card);
    }
    pub fn remove_card(&mut self, index: u32) -> Result<Carta,String>{
        
        if (index as usize) < self.cards.len(){
            Ok(self.cards.remove(index as usize))
        }else {
            Err(String::from("indice escolhido invalido"))
        }
    }
    fn set_card(&mut self, card: Vec<Carta>) {
        self.cards = card;
    }
    pub fn get_cards(&self) -> Vec<Carta>{
        self.cards.clone()
    }
    pub fn get_card(&mut self, index: u32) -> Result<Carta,String>{
        if (index as usize) < self.cards.len() {
            return Ok(self.cards[index as usize].clone());
        }
        Err(String::from("indice escolhido invalido"))
    }
    pub fn print_card(&self){
        for (i,card) in self.cards.iter().enumerate(){
            println!("{}->{:?}", i, card.get_carta());
        }
    }

    //Name
    fn set_name(&mut self, name: String){
        self.name = name;
    }
    pub fn get_name(&self) -> String{
        self.name.to_string()
    }

    //Mesa
    pub fn set_new_table(&mut self, new_table: Vec<Carta>){
        self.mesa.push(new_table);
    }
    pub fn add_card_table(&mut self, local: bool, mesa: u32, card: Carta){
        if local{
            self.mesa[mesa as usize].push(card);
        }else{
            self.mesa[mesa as usize].insert(0,card);
        }
    }
}