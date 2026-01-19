mod analyzer;
mod ollama_chat;
use std::fs;

fn main() {
    //let file_path = "sample.log";
    let file_path = "ediingress.log";
    let log_text = fs::read_to_string(file_path);
    let log_data = match log_text{
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading log file: {}", e);
            return;
        }

    };

    let messages = analyzer::build_rca_messages(&log_data);
    let model = "llama3.1:latest";
    let result = ollama_chat::chat_blocking(model, messages);
    match result {
        Ok(report) => {
            println!("=== RCA REPORT ===\n");
            println!("{}", report);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Debug: {:?}", e);
        }
    }
    println!("Hello, world!");
}
