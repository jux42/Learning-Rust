use std::ops::Add;

pub fn variables_exo() {
    let x:f64 = 5.2;

    {
        let x = x + 1.0;

        {
            let x = x * 2.0;
            println!("La valeur de x dans la portée interne est : {}", x);
        }

        println!("La valeur de x dans la portée intermédiaire est : {}", x);
    }

    println!("La valeur de x est : {}", x);

    let chat_aux_yeux_de_coeur = '😻';

    println!("{}", chat_aux_yeux_de_coeur);

    let tup = (500, 8.4, 1);

    let (_x, y, _z) = tup;

    println!("La valeur de y+y est : {}", tup.1+y);
    let word = this_is_a_test(String::from("youyou"));
    println!("{}", word.add("  YOOOOOOOO"));


}

pub fn this_is_a_test(mot: String) -> String {
    let texte: String =  mot.add("TEST JUXXXXX");
    texte.add("321321321321")
}