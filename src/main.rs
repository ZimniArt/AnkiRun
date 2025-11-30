use std::usize;
use std::{collections::HashMap, fs, os::windows::io as win_io, str::SplitWhitespace};
use rtranslate::translate;
use regex::Regex;

use std::io::{self, Write};
use std::fmt::Display;
// use lindera::tokenizer::Tokenizer;
// use lindera_ipadic::builder::IpadicBuilder;

enum Language{
    English,
    Russian,
    Japanese,
}

impl Language{
    fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Russian => "ru",
            Language::Japanese => "ja",
            
        }
    }
} 

fn input(prompt: &str)-> String {
    print!("{}",prompt);
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();;
    s.trim().to_string()
}
fn input_options <T: Display> (prompt: &str, options: &[T]) ->usize{
    loop {
        println!("{}", prompt);

        for (i,opt) in options.iter().enumerate(){
            println!(" {}. {}", i+1, opt);
        }
        print!("Enter number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<usize>(){
            if num >= 1 && num <= options.len(){
                return num - 1
            }
        }
        println!("invalid input, choose correct number:\n");
    }
}

fn main() {
    
    // import file
    let input_path = String::from("D:/2_projects/9_rust/Input_text.txt");
    let output_path : String = String::from("D:/2_projects/9_rust/Output_text.txt");
    let input_text = fs::read_to_string(input_path).expect("no input text"); 
    let _output_percentage: f64 = 0.05;
    let _input_lang = Language::English.code();
    let _output_lang = Language::Japanese.code();
    let _ignore_word_list = vec![
        //English
        "a", "an", "the",
        "and", "or", "but",
        "in", "on", "at", "to", "of", "for", "with",
        "is", "are", "was", "were", "be", "been", "being",
        "have", "has", "had", "do", "does", "did",
        //Russian
        // Prepositions
        "в", "во", "на", "по", "о", "об", "от", "до", "за", "из", "изо", "со", "с",
        "у", "при", "через", "среди", "между", "без", "для", "над", "под",
        // Conjunctions
        "и", "да", "но", "а", "или", "либо", "хотя", "когда", "если", "потому", "чтобы", "хоть",
        // Particles
        "же", "ли", "бы", "ведь", "уж", "лишь", "даже", "вот", "мол", 
        // Function words
        "не", "ни",
         // Copula verbs
        "есть", "был", "была", "было", "были", "будет", "будут",
    ];
   

    let mut _working_text: String = String::from(&input_text);

    let _prepared_text: SplitWhitespace<'_>= _working_text.split_whitespace(); 
    let mut _frequency_hashmap: HashMap<String, i32> = create_frequency_hashmap(_prepared_text, _ignore_word_list);
    let mut _sorted: Vec<(String, i32)> = _hashmap_to_sorted_list(_frequency_hashmap); 
    dbg!(_sorted.len());
    _sorted = shorten_dictionary(_sorted, _output_percentage);
    dbg!(_sorted.len());
    let mut _sorted_dictionary: Vec<(String,String)> = translate_words(_sorted, _input_lang, _output_lang);
    let dictionary : HashMap<String, String> = _sorted_dictionary.into_iter().collect(); 
    let mut _output_text: String = translate_using_custom_dictionary(&input_text, dictionary); 
    
    //export file 
    fs::write(output_path, _output_text).expect("couldnt output file...");    
}



fn create_frequency_hashmap(text: SplitWhitespace<'_>, ignore_word: Vec<&str> )-> HashMap<String, i32>{
    let mut hashmap: HashMap<String, i32> = HashMap::new() ;
    for word in text {
        if ignore_word.contains(&word) {
            continue;
        }
        *hashmap.entry( word.to_string()).or_insert( 0) += 1;
    } 
    hashmap
}
fn _hashmap_to_sorted_list(hashmap: HashMap<String,i32>) -> Vec<(String, i32)>{
    let mut _new_hashmap: Vec<(String, i32)> = hashmap.into_iter().collect();
    _new_hashmap.sort_by(|a, b| b.1.cmp(&a.1));
    _new_hashmap
}
fn shorten_dictionary(dictionary: Vec<(String, i32)>, remaining_percent: f64) ->Vec<(String,i32)>{
    let mut _new_dictionary = dictionary;
    let _dict_lenght: usize = (_new_dictionary.len()as f64 * remaining_percent).ceil() as usize;
    _new_dictionary.truncate(_dict_lenght);
    //percent logic
    _new_dictionary
}
fn translate_words(word_list: Vec<(String, i32)>, input_lang: &str, output_lang: &str) -> Vec<(String,String)>{

    let _word_count: usize = word_list.len();
    let mut _itteration: i32 = 0;

    let _translated_list = word_list
        .iter()
        .enumerate()
        .map(|(i,(word, _count))|{
            let translated= match translate(&word, input_lang,output_lang)  {
                Ok(translated) =>  translated,
                Err(_err) =>word.clone(),
            };
            progress_count("Translation progress", i+1, _word_count);
            (word.clone(),translated)
        })
        .collect();
        _translated_list
    }
fn progress_count(name: &str,current_progress: usize, total: usize){
    print!("\r {}: {} /{}",name, current_progress, total);
        std::io::Write::flush(&mut std::io::stdout()).unwrap(); 
}
fn translate_using_custom_dictionary (text: &String, dictionary: HashMap<String, String>)->String{
    let re = Regex::new(r"\b\w+\b").unwrap();
    let mut _new_text: String = String::new(); 
    _new_text =  re.replace_all(&text, |caps: &regex::Captures| {
        let word = &caps[0];
        dictionary.get(word).cloned().unwrap_or_else(|| word.to_string())
    }).to_string();
    _new_text
}