use std::fs::read_to_string;
use std::io::Read;
use rand::Rng;
use std::io::Write;

// create module

// const for the number of word in the terminal
pub const nbOfWord: usize = 10;

pub fn getWordsAndPassword(wordSize: &usize) -> (Vec<String>, usize){
    // Get list of names
    let words = getRandomWords(wordSize);

    // display the list of words
    println!("Liste des mots possible : ");
    displayWords(&words);

    let password = getPassword(&words);

    return (words, password);
}


// --------------------------------------------------
// -------------- Recover chosen word ---------------
// --------------------------------------------------

fn getRandomWords(lengthWord: &usize) -> Vec<String> {
    // read file and store its content
    let path = format!("wordList/{lengthWord}Letter.txt");
    let content =  read_to_string(path).expect("Should have been able to read the file");
    let lines: Vec<&str> = content.split("\r\n").collect();

    // get random number as word index in lines
    let mut wordIndexList: [usize; nbOfWord] = [0; nbOfWord];

    let mut rng = rand::thread_rng();

    for i in (0..nbOfWord){
        wordIndexList[i] = rng.gen_range(0..lines.len())
    }

    // get the random words
    let mut randomWordList: Vec<String> = Vec::new();

    for i in (0..nbOfWord) {
        randomWordList.push(String::from(lines[wordIndexList[i]]));
    }

    randomWordList
}


pub fn displayWords(words: &Vec<String>){
    for i in (0..words.len()){
        println!("{}. {}", i+1, words[i])
    }
}


pub fn askPlayer(words: &Vec<String>) -> String{
    let mut index: usize = 0;
    let mut joker = false;

    let word: String;

    while(index == 0) && joker == false{
        // Ask user
        print!("Choisissez un mot (1, 2, 3, ...), joker 'j' ou quitter 'q' : ");
        std::io::stdout().flush().unwrap();

        // Get answer
        let mut answer = String::from("");
        std::io::stdin().read_line(&mut answer).expect("Failed to read line");

        // extract chosen word
        let realAnswer = answer.split("\r\n").collect::<Vec<&str>>()[0];

        let firstLetter = realAnswer.chars().collect::<Vec<char>>()[0];
        if firstLetter == 'q'{
            std::process::exit(0);
        } else {
            if firstLetter == 'j'{
                joker = true;
            } else {
                index = match realAnswer.parse::<usize>() {
                    Ok(i) => {
                        if i >= 1 && i <= nbOfWord {
                            i
                        } else {
                            println!("Veuillez choisir un nombre entre 1 et {nbOfWord} compris puis appuyer sur 'Entrer'");
                            0
                        }
                    },
                    Err(_e) => {
                        println!("Veuillez choisir un nombre entre 1 et {nbOfWord} compris puis appuyer sur 'Entrer'");
                        0
                    },
                };
            }
        }
    }

    if ! joker {
        word = String::from(&words[index-1]);
    } else {
        word = String::from("");
    }

    return word;
}


fn getPassword(words: &Vec<String>) -> usize{
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());

    return index;
}