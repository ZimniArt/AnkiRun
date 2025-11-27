fn main() {
    let config: Config  = Config{
        input_language: "en".to_string(),
        output_language: "jp".to_string(),
        start_percent: 0.0,
        end_percent: 20.0,
        input_file_path:"text.txt".to_string(),
    };
    
   
    println!("{} {} {} {} {}",
    config.input_language,
    config.output_language,
    config.start_percent, 
    config.end_percent,
    config.input_file_path);
    // 1. read config
    // 2. load file
    // 3. analyze vocabulary
    // 4. translate vocabulary
    // 5. rebuild text with progression
    // 6. export

    //check
}
 
struct Config{
    input_language: String,
    output_language: String,
    input_file_path: String,
    start_percent: f32,
    end_percent: f32,
}
