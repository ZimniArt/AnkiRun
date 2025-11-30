use std::{collections::HashMap, default, fs, hash::Hash, str::SplitWhitespace};
use rtranslate::translate;
use regex::Regex;
fn main() {
    
    // import file
    let input_path = String::from("D:/2_projects/9_rust/Input_text.txt");
    let output_path : String = String::from("D:/2_projects/9_rust/Output_text.txt");
    let input_text = fs::read_to_string(input_path).expect("no input text"); 
    let _output_percentage: f64 = 0.30;
    let _input_lang = "en";
    let _output_lang = "ja";
    let _ignore_word_list = vec!["the", "a"];


    let mut _working_text: String = String::from(&input_text);
    let _prepared_text: SplitWhitespace<'_>= _working_text.split_whitespace(); 
    let mut _frequency_hashmap: HashMap<String, i32> = create_frequency_hashmap(_prepared_text, _ignore_word_list);
    let mut _sorted: Vec<(String, i32)> = _hashmap_to_sorted_list(_frequency_hashmap); 
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

fn translate_words(word_list: Vec<(String, i32)>, input_lang: &str, output_lang: &str) -> Vec<(String,String)>{
    let _translated_list = word_list.iter().map(|(word, _count)|{
        (word.clone(), match translate(&word, input_lang,output_lang)  {
            Ok(translated) => translated,
            Err(_err) =>word.clone(),})
        } ).collect();
        _translated_list
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