pub fn build_review_prompt(_language: &str, code: &str)-> String{
    /*
    Why we did it this way (no chaining)

String::new(), push_str() are explicit

You can see how prompt is built

Later we’ll refactor into cleaner chaining/formatting
     */
    let mut prompt = String::new();
    prompt.push_str("You are a senior software engineer.\n");
    prompt.push_str("Review the following code and respond in this format:\n");
    prompt.push_str("1) Summary\n");
    prompt.push_str("2) Potential Issues\n");
    prompt.push_str("3) Suggestions\n");
    prompt.push_str("4) Risk Level (Low/Medium/High) with one-line reason\n\n");
    prompt.push_str("Language: ");
    prompt.push_str("\n\n");
    prompt.push_str("Code:\n");
    prompt.push_str(code);
    prompt.push_str("\n");
    prompt
    
}