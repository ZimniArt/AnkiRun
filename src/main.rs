use std::fs;

fn main() {
   
    // import file
    let input_path = String::from("D:/2_projects/9_rust/Input_text.txt");
    let input_text = fs::read_to_string(input_path); 
    //copy to new String
    let output_text: String = match input_text {
        Ok(text) => text,
        Err(_) => String::from("no file path provided"),
    };

    
    //export file
    let output_path : String = String::from("D:/2_projects/9_rust/Output_text.txt");

    fs::write(output_path, output_text).expect("couldnt output file...");    
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
