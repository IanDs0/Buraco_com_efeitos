mod carta;
mod user;

mod enums;
// use enums::efeito::Efeito;
// use enums::nipe::Nipe;

mod jogo;
use jogo::jogo::Jogo;

use std::io;

fn main() {

    
    let mut jogo = Jogo::new();

    //colocar pontos e HP como constantes no jogo.rs
    jogo = match jogo.set_user("Ian LL Honorio".to_string(), 10, 10){
        Ok(c) => c,
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };
    jogo = match jogo.set_user("Sarah AL Honorio".to_string(), 10, 10){
        Ok(c) => c,
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };
    jogo.players[0].pegar_lixo();
    jogo.players[0].print_user();
    // let confima:String = match jogo.players[0].pegar_lixo(){
    //     Ok(c) => c,
    //     Err(e) => {
    //         println!("Erro: {}", e);
    //         return;
    //     }
    // };
    // println!("{}", confima);

    /* match jogo.pull_cards(0){
        Ok(c) => {
            println!("{}", c);
        },
        Err(e) => {
            println!("Erro: {}", e);
        }
    } */

    let mut entrada: String = String::new();
    let mut array: Vec<u32> = Vec::new();
                                    
    let mut encontro: [bool; 4] = [
        false,//sequence number
        false,//2 + sequence number
        false,//number + 2 + number
        false,//sequence number + 2
    ];
    
    for _ in 0..3{
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error reading");

        array.push(entrada.trim().parse().unwrap());
        entrada.clear();
    }

    match jogo.down_3_cards_letter(0, array){
        Ok(jogo) => {
            
            println!("Deseja inserir as cartas em qual forma?");

            if jogo[0] == true{
                println!("Em sequencia!");
            }
            if jogo[1] == true{
                println!("Coringa no inicio!");
            }
            if jogo[2] == true{ 
                println!("Coringa no meio!");
            }
            if jogo[3] == true{
                println!("Coringa no fim!");
            }



            return;
        },
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };

    // println!("{:?}",encontro);

    // jogo.players[0].print_user();
    

}
