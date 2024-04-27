use rand::Rng;

pub fn branches_exo() {
    let mut chiffre = rand::thread_rng().gen_range(1..=6);

    while chiffre != 50 {
        let nombre = 2 * chiffre;
        if nombre > 10 {
            println!("oui oui oui");
        } else {
            println!("non non non");
        }
        chiffre = if nombre > 90 { 100 } else if nombre > 40 { 50 } else { 0 };


        chiffre = rand::thread_rng().gen_range(1..=50);
        println!("{}", chiffre)
    }
}

