use std::fs::read_to_string;
use std::io::Read;
use rand::Rng;

// struct Terminal{
//
// }


fn main(){
    getRandomWords(5);
}

fn getRandomWords(lengthWord: u16) -> Vec<String> {
    // const for the number of word in the terminal
    const nbOfWord: usize = 10;



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


    for line in &randomWordList{
        println!("{line}");
    }

    randomWordList
}