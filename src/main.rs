use std::fs::read_to_string;
use std::io::Read;
use rand::Rng;
use std::io::Write;

// struct Terminal{
//
// }

// const for the number of word in the terminal
const nbOfWord: usize = 10;

fn main(){
    // Get list of names
    let words = getRandomWords(5);

    // display the list of words
    println!("Liste des mots possible : ");
    displayWords(&words);

    // Ask player
    let word = askPlayer(&words);
}

fn getRandomWords(lengthWord: u16) -> Vec<String> {
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


fn displayWords(words: &Vec<String>){
    for i in (0..words.len()){
        println!("{}. {}", i+1, words[i])
    }
}


fn askPlayer(words: &Vec<String>){
    let mut index: usize = 0;

    while(index == 0){
        // Ask user
        print!("Choisissez un mot (1, 2, 3, ...) : ");
        std::io::stdout().flush().unwrap();

        // Get answer
        let mut answer = String::from("");
        std::io::stdin().read_line(&mut answer).expect("Failed to read line");

        // extract chosen word
        let realAnswer = answer.split("\r\n").collect::<Vec<&str>>()[0];


        index = match realAnswer.parse::<usize>() {
            Ok(0) => {
                println!("Veuillez choisir un nombre entre 1 et {nbOfWord} puis appuyer sur 'Entrer'");
                0
            },
            Err(_e) => {
                println!("Veuillez choisir un nombre entre 1 et {nbOfWord} puis appuyer sur 'Entrer'");
                0
            },
            Ok(i) => i,
        };
    }

    let word = &words[index-1];

    print!("{word}")
}