use std::{collections::HashMap, fs, str::SplitWhitespace};
use rtranslate::translate;

fn main() {
   
    // import file
    let input_path = String::from("D:/2_projects/9_rust/Input_text.txt");
    let input_text = fs::read_to_string(input_path).expect("no input text"); 

    let mut _working_text: String = String::from(&input_text);
    let exlude_words = vec!["the", "a"];


    //sorting text file
    let _words: SplitWhitespace<'_>= _working_text.split_whitespace();
    let mut _counts: HashMap<String, i32> = HashMap::new();

    for word in _words {
        if exlude_words.contains(&word) {
            continue;  // skip these words
        }
        *_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut _sorted: Vec<(String, i32)> = _counts.into_iter().collect();
    _sorted.sort_by(|a, b| b.1.cmp(&a.1));

    // let mut _sorted_dictionary: Vec<(String,String)> = _sorted.iter().map(|(word, _count)|{
    // (word.clone(), word.clone())
    // } ).collect();
    let mut _sorted_dictionary: Vec<(String,String)> = _sorted.iter().map(|(word, _count)|{
    (word.clone(), match translate(&word, "en","ja")  {
        Ok(translated) => translated,
        Err(_err) =>word.clone(),})
    } ).collect();
    println!("{:?}",_sorted_dictionary);
    //create output file. sorted list
    let mut _output_text: String = String::new(); 
   
    for (_word, translation) in _sorted_dictionary{
        _output_text.push_str(&format!("{_word}: {translation}\n"));
    }
    //translate text
    
    // _output_text = match translate(&_output_text, "en","ja")  {
    //     Ok(translated) => translated,
    //     Err(_err) =>_output_text,
    // };
 
    //export file
    let output_path : String = String::from("D:/2_projects/9_rust/Output_text.txt");
    fs::write(output_path, _output_text).expect("couldnt output file...");    
}

//imputs 
    //input language
    //output language
    //progression rate start%
    //progression rate end%
    //file 
//sort txt by popularity
//exlcude all non word : a,the, of , or, as,hmm, .<>?!!%$^&!@%()
//translate list to dictionary of popularity words english word : Japanese ) 
//translate text based on lenght of text and required progress, with create dictionary     
//export txt

// 1. read config
// 2. load file

// 3. analyze vocabulary
// 4. translate vocabulary
// 5. rebuild text with progression
// 6. export
