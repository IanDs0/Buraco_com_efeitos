mod carta;
use carta::cartas::Carta;

mod user;
use user::users::User;

mod enums;
// use enums::efeito::Efeito;
// use enums::nipe::Nipe;

mod jogo;
use jogo::jogo::Jogo;

fn main() {

    let card: (String, u32, bool) = (
        String::from("Espadas"),
        9,
        true,
    );

    let carta = match Carta::new(&card.0, card.1, card.2) {
        Ok(c) => c,
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };



    let card: (String, u32, bool) = (
        String::from("Copas"),
        6,
        true,
    );

    let carta2 = match Carta::new(&card.0, card.1, card.2) {
        Ok(c) => c,
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };





    let mut user = User::new();

    let vec_carta = vec![carta];




    user.set_user(
        "Ian".to_string(),
        10,
        10,
        vec_carta
    );

    print!("{} points\n",user.get_points());//Ok
    print!("{} hp\n",user.get_hp());//Ok

    user.add_card(carta2);//Ok

    match user.remove_card(1){
        Ok(c) => println!("{}",c),
        Err(e) => println!("Erro: {}", e)
    };//Ok

    let aux = user.clone().get_card();//Ok

    for ve in aux.iter() {
        ve.print_carta();
    }//Ok

    user.add_points(3);//Ok
    user.damage_hp(9);//Ok
    user.health_hp(5);//Ok
    
    println!("Nome: {}",user.get_name());//Ok
    


    user.print_user();//Ok

    print!("{:?}",user.get_user());//Ok

}
