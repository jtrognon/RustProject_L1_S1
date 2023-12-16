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

    let (mut words, passwordId) = passwordAndWords::getWordsAndPassword(&wordSize);

    let quit = false;
    let mut attempt: usize = 4;

    // Init var loop
    let mut chosenWord = &passwordAndWords::Word::init(String::from(""));
    let mut matchingPair: usize = 0;
    let mut rng = rand::thread_rng();
    let mut password = match words.get(passwordId){
        Some(i) => i,
        _ => panic!("Problème lors de la création du mot de passe")
    };
    let mut joker = rng.gen_range(0..4);

    let mut wordIndex: usize;

    let mut alreadySelected: Vec<&passwordAndWords::Word> = Vec::new();
    let mut luck: u16;
    while quit == false {

        // display the list of words
        println!("----------------------------------------------------");

        println!("Liste des mots possible : ");
        displayWords(&words, &alreadySelected);

        println!("---- Vous avez {attempt} essais et {joker} jokers ----");

        wordIndex = passwordAndWords::askPlayer(&words);
        if wordIndex == 0{
            if joker > 0 {
                joker -= 1;
                luck = rng.gen_range(0..100);

                if luck <= 25{
                    attempt += 1;
                    println!("Vous avez un essai en plus")
                } else if luck <= 90 {
                    alreadySelected = randomWord(&words, alreadySelected)
                }
            }
        } else {
            wordIndex -= 1;
            chosenWord = words.get(wordIndex).unwrap();

            alreadySelected.push(chosenWord);

            attempt -= 1;

            matchingPair = passwordAndWords::matchingLetters(&chosenWord, & password);

            println!("Nombre de lettres correspondantes : {matchingPair}/{}", wordSize);

            if matchingPair == wordSize{
                println!("--------------------------------------");
                println!("Bravo !!! Il vous restait {attempt} essais et {joker} jokers.");
                println!("--------------------------------------");
                std::process::exit(0);
            }

            if attempt == 0{
                println!("---------------------------------------------------");
                println!("Vous avez perdu !!! Le mot de passe était : {}.", password.getContent());
                println!("---------------------------------------------------");
                std::process::exit(0);
            }
        }
    }
}



fn displayWords(words: &Vec<passwordAndWords::Word>, alreadySelected: &Vec<&passwordAndWords::Word>){
    let mut wordPrint: String;
    let mut word: &passwordAndWords::Word;
    for i in (0..words.len()){
        word = words.get(i).unwrap();
        if word.inList(alreadySelected){
            wordPrint = String::from("...");
        } else {
            wordPrint = word.getContent();
        }

        println!("{}. {}", i+1, wordPrint)
    }
}


fn randomWord<'a>(words: &'a Vec<passwordAndWords::Word>, mut alreadySelected: Vec<&'a passwordAndWords::Word>) -> Vec<&'a passwordAndWords::Word>{
    let mut word: &passwordAndWords::Word;

    let mut wordNotSelected = Vec::new();
    for i in (0..words.len()){
        word = words.get(i).unwrap();
        if ! word.inList(&alreadySelected) {
            wordNotSelected.push(word);
        }
    }

    let mut rng = rand::thread_rng();
    let randomNum = rng.gen_range(0..wordNotSelected.len());

    word = words.get(randomNum).unwrap();

    if word.isPassword() == &true{
        alreadySelected
    } else {
        alreadySelected.push(word);
        alreadySelected
    }
}






// PAS TROP DE ALWAYS