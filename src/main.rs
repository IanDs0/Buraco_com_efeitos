mod carta;
mod user;

mod enums;
// use enums::efeito::Efeito;
// use enums::nipe::Nipe;

mod jogo;
use jogo::jogo::Jogo;

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

    jogo.players[0].print_user();
    jogo.players[1].print_user();
    

}
