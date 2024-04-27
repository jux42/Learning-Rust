use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn find() {
    'outer: loop {
        let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
        println!("devinez à quel nombre je pense ! (DEV = c'est {}", secret_num);
        loop {
            println!("saisissez votre idée");
            let mut idee = String::new();

            io::stdin()
                .read_line(&mut idee)
                .expect("une erreur est survenue");

            let idee: u32 = match idee.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("vous devez saisir un nombre !!");
                    continue;
                }
            };

            println!("votre nombre : {}", idee);

            match idee.cmp(&secret_num) {
                Ordering::Less => println!("c'est +"),
                Ordering::Greater => println!("c'est -"),
                Ordering::Equal => {
                    println!("BRAVOOOOOOOOO");
                    break 'outer;
                }
            }
        }
    }
}