mod ollama;
mod reviewer;
use std::fs;
fn main() {
    println!("Hello, world!");
    let file_path = "src/sample.rs";
    let code=fs::read_to_string(file_path);
    let code_text = match code{
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            return;
        },
    };
    //extracted the code from file as a string
    // Step C: Build prompt deterministically
    let prompt = reviewer::build_review_prompt("Rust", &code_text);
    // Step D: Call Ollama generate
    let model = "llama3.1:latest";
    let result = ollama::generate_code_review(&model, &prompt);
    // Step E: Print output
    match result{
        Ok(response) =>{
            println!("Code Review Response:\n{:?}", response);
        },
        Err(e)=>{
            eprintln!("Error generating code review: {}", e);
        }
    }
}
