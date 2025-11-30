use std::fs::File;
use std::{process, usize};
use std::{collections::HashMap, fs, os::windows::io as win_io, str::SplitWhitespace};
use rtranslate::translate;
use regex::Regex;
use rfd;

use std::io::{self, Write};
use std::fmt::{Display, write};
use std::path::Path;

enum Language{
    English,
    Russian,
    Japanese,
}

impl Language {
    pub fn  as_str(&self) -> &'static str{
        match self {
        Language::English => "en",
        Language::Russian => "ru",
        Language::Japanese =>"ja",
        }
    }
}

impl Display for Language{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->std::fmt::Result{
        write!(f, "{}", self.as_str())
    }
} 

fn input(prompt: &str)-> i32 {
    loop{
        print!("{}",prompt);
        io::stdout().flush().unwrap();
    
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        
        match s.trim().parse::<i32>() {
            Ok(num) =>return  num,
            Err(_) => println!("invalid number, try again"),
            
        }
    }
    
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
    
    //let input_path = String::new();
    //let _file = rfd::FileDialog::new().add_filter("Text files",&["txt"]).pick_file();
//
     let input_path : String = loop {
       if let Some(path) = rfd::FileDialog::new()
       .add_filter("Text files", &["txt"])
       .pick_file()
       {
        break path.to_string_lossy().into_owned();
       }
       println!("no valid file selected, exiting programm");
       process::exit(0);
    };
   
    // import file
    let output_path : String = {
        let path = Path::new(&input_path);
        let stem = path.file_stem().unwrap().to_string_lossy();
        let ext = path.extension().unwrap_or_default().to_string_lossy();
        let file_name= format!("{}_translated.{} ", stem, ext);
        path.parent()
        .unwrap_or_else(|| Path::new("."))
        .join(file_name)
        .to_string_lossy()
        .into_owned() 
    };


    let input_text = fs::read_to_string(input_path).expect("no input text"); 
    
    let _languages = vec![Language::English, Language::Russian, Language::Japanese];
    
    let _input_index = input_options("Whats the language of the file?", &_languages);
    let _input_lang = _languages[_input_index].as_str();
    let _input_index = input_options("What language translate to?", &_languages);
    let _output_lang = _languages[_input_index].as_str();
    let _output_percentage: f64 = input("What percent of words need to be translated? 1..100 :") as f64 *0.01 ;
    
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