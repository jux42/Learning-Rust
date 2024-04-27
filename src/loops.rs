pub fn loops_exo() {
    let mut counter = 0;

    let testing: String = loop {
        counter += 1;
        println!("loop ! {counter}");

        if counter == 100 {
            break String::from("OK !");
        }
    };
    println!("{testing}");

    let mut testing_string = String::from("THIS IS A TEST");
    println!("{testing_string}");

    counter = test_counter(counter, &mut testing_string);
    println!("{testing_string}");

    fn test_counter(mut nombre: i32, texte: &mut String) -> i32 {
        texte.push_str("bla bla");
        nombre * 5
    }
}