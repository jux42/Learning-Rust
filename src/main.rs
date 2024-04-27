mod variables;
mod branches;
mod loops;
mod slices;
mod structures;
mod secret_num_finder;

use rand::Rng;
use structures::User;

fn main() {

let mut trigger: bool = true;

if trigger==true {
    secret_num_finder::find();
    variables::variables_exo();
    branches::branches_exo();
    loops::loops_exo();
}

let mut text = "KING JUJU IS THE BEST";
let first_slice =  slices::slice_exo(text);
    println!("first word is {}", first_slice);

let mut juju = User {
        email: String::from("juju@example.com"),
        pseudo: String::from("Some Kind Of Juju"),
        actif: true,
        nombre_de_connexions: 6543213,
    };
    print!("==> {0}\n{1}\n{2}\n", juju.pseudo, juju.email, juju.nombre_de_connexions);
    juju.pseudo = String::from("Another Kind Of Juju");
    println!("==> {}", juju.pseudo);
}