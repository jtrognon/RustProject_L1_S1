use std::io::Write;
use rand::Rng;
use crate::passwordAndWords::nbOfWord;

// Import file
mod passwordAndWords;

fn main() {
    // Choice of word length
    print!("Veuillez choisir la taille des mots (de 5 à 10 compris) : ");
    std::io::stdout().flush().unwrap();

    // Get word length
    let mut answer = String::from("");
    std::io::stdin().read_line(&mut answer).expect("Failed to read line");

    // extract chosen word length
    let realAnswer = answer.split("\r\n").collect::<Vec<&str>>()[0];

    let mut wordSize: usize = 0;

    while(wordSize == 0) {
        wordSize = match realAnswer.parse::<usize>() {
            Ok(i) => {
                if i >= 5 && i <= 10 {
                    i
                } else {
                    println!("Veuillez choisir un nombre entre 5 et 10 compris puis appuyer sur 'Entrer'");
                    0
                }
            },
            Err(_e) => {
                println!("Veuillez choisir un nombre entre 5 et 10 compris puis appuyer sur 'Entrer'");
                0
            },
        };
    };

    // -----------------------------------------------------

    let (words, passwordId) = passwordAndWords::getWordsAndPassword(&wordSize);

    let quit = false;
    let mut attempt: usize = 4;

    // Init var loop
    let mut chosenWord = String::from("");
    let mut matchingPair: usize = 0;
    let mut rng = rand::thread_rng();
    let password = match words.get(passwordId){
        Some(i) => i,
        _ => panic!("Problème lors de la création du mot de passe")
    };
    let joker = rng.gen_range(0..4);

    while quit == false {
        println!("----------------------------------------------------");
        println!("Vous avez {attempt} essais et {joker} jokers");
        chosenWord = passwordAndWords::askPlayer(&words);

        if chosenWord == "" {
            attempt += 1;
        } else {
            attempt -= 1;
        }

        matchingPair = matchingLetters(&chosenWord, &password);

        println!("Nombre de lettres correspondantes : {matchingPair}/{}", wordSize);

        if matchingPair == wordSize{
            println!("--------------------------------------");
            println!("Bravo !!! Il vous restait {attempt} essais et {joker} jokers.");
            println!("--------------------------------------");
            std::process::exit(0);
        }

        if attempt == 0{
            println!("---------------------------------------------------");
            println!("Vous avez perdu !!! Le mot de passe était : {password}.");
            println!("---------------------------------------------------");
            std::process::exit(0);
        }
    }
}

fn matchingLetters(chosenWord: &String, password: &String) -> usize{
    let mut matchingPair: usize = 0;
    for i in (0..chosenWord.len()){
    if chosenWord.chars().collect::<Vec<char>>()[i] == password.chars().collect::<Vec<char>>()[i]{
            matchingPair += 1;
        }
    }

    matchingPair
}