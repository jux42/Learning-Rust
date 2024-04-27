use rand::Rng;

use structures::create_user;

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

    print!("==> {0}\n{1}\n{2}\n", juju.pseudo, juju.email, juju.nombre_de_connexions);
    juju.pseudo = String::from("Another Kind Of Jux");
    println!("==> {}", juju.pseudo);
}