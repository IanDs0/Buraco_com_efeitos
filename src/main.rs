mod carta;
use carta::cartas::Carta;

mod user;
use user::users::User;

fn main() {

    let card: (String, u32, bool) = (
        String::from("Espadas"),
        13,
        true,
    );

    let carta = match Carta::new(&card.0, card.1, card.2) {
        Ok(c) => c,
        Err(e) => {
            println!("Erro: {}", e);
            return;
        }
    };

    carta.print_carta();




    let mut user = User::new();

    let vec_carta = vec![carta];

    user.set_user(
        "Ian".to_string(),
        10,
        5,
        vec_carta
    );

    user.print_user();

    let mut damage: u16 = 6;

    user.damage_hp(damage);
}
