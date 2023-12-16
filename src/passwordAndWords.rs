use std::fs::read_to_string;
use std::io::Read;
use rand::Rng;
use std::io::Write;

// create module

// const for the number of word in the terminal
pub const nbOfWord: usize = 10;

pub struct Word {
    content: String,
    isPassword: bool,
}

impl Word{
    fn setPassword(&mut self){
        self.isPassword = true;
    }

    pub fn init(content: String) -> Word{
        Word{content, isPassword: false}
    }


    pub fn getContent(&self) -> String {
        String::from(&self.content)
    }


    pub fn inList(&self, list: &Vec<&Word>) -> bool{
        let mut res = false;
        for word in list{
            if &word.content == &self.content{
                res = true
            }
        }

        return res;
    }

    pub fn isPassword(&self) -> &bool{
        &self.isPassword
    }
}

pub fn getWordsAndPassword(wordSize: &usize) -> (Vec<Word>, usize){
    // Get list of names
    let mut words = getRandomWords(wordSize);


    let password = getPassword(&words);
    words.get_mut(password).unwrap().setPassword();


    return (words, password);
}


// --------------------------------------------------
// -------------- Recover chosen word ---------------
// --------------------------------------------------

fn getRandomWords(lengthWord: &usize) -> Vec<Word> {
    // read file and store its content
    let path = format!("wordList/{lengthWord}Letter.txt");
    let content =  read_to_string(path).expect("Should have been able to read the file");
    let lines: Vec<&str> = content.split("\r\n").collect();

    // get random number as word index in lines
    let mut wordIndexList: [usize; nbOfWord] = [0; nbOfWord];

    let mut rng = rand::thread_rng();

    let mut randomIndex;
    for i in (0..nbOfWord){
        randomIndex = rng.gen_range(0..lines.len());

        while wordIndexList.contains(&randomIndex){
            randomIndex = rng.gen_range(0..lines.len());
        }

        wordIndexList[i] = randomIndex;
    }

    // get the random words
    let mut randomWordList: Vec<Word> = Vec::new();

    for i in (0..nbOfWord) {
        randomWordList.push(Word::init(String::from(lines[wordIndexList[i]])));
    }

    randomWordList
}

pub fn askPlayer(words: &Vec<Word>) -> usize{
    let mut index: usize = 0;
    let mut joker = false;

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

    return index;
}


fn getPassword(words: &Vec<Word>) -> usize{
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());

    return index;
}



pub fn matchingLetters(chosenWord: &Word, password: &Word) -> usize{
    let mut matchingPair: usize = 0;
    for i in (0..chosenWord.content.len()){
        if chosenWord.content.chars().collect::<Vec<char>>()[i] == password.content.chars().collect::<Vec<char>>()[i]{
            matchingPair += 1;
        }
    }

    matchingPair
}