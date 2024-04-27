use rand::Rng;

use structures::create_user;
use structures::User;


mod variables;
mod branches;
mod loops;
mod slices;
mod structures;
mod secret_num_finder;

fn main() {
    let mut trigger: bool = false;

    if trigger == true {
        secret_num_finder::find();
        variables::variables_exo();
        branches::branches_exo();
        loops::loops_exo();
    }

    let mut text = "KING JUJU IS THE BEST";
    let first_slice = slices::slice_exo(text);
    println!("first word is {}", first_slice);

    let mut juju = create_user(
        String::from("Some Kind Of Jux"),
        String::from("Jux@gmail.com"),
    );

    let mut juju2 = create_user(
        juju.pseudo.clone(), // clonage pour maintenir l'utilisation de juju
        String::from("Jux2@gmail.com"),
    );
    print!("==> {}\n{}\n{}\n", juju.pseudo, juju.email, juju.nombre_de_connexions);
    println!("\n==> {}\n{}", juju2.pseudo, juju2.email);


    let mut juju3 = User{
        pseudo: String::from("jux3"),
        ..juju //à ce stade juju n'est plus utilisable car sa possession est transmise à juju3
    };

    juju.pseudo = String::from("Another Kind Of Jux"); //nouvelle valeur pour juju.pseudo, qui redevient utilisable (mais pas juju.email)
    println!("==> {}", juju.pseudo);
    println!("\n==> {}\n{}", juju3.pseudo, juju3.email);


}