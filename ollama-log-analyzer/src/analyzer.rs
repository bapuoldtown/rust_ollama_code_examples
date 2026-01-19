pub fn build_rca_messages(log_text: &str) -> Vec<(String, String)> {
    let mut messages: Vec<(String, String)> = Vec::new();
    let system = String::from(
        "You are a senior SRE. You analyze logs and produce a structured RCA report.
Return markdown with sections:
1) Summary
2) Suspected Root Cause
3) Evidence (quote exact log lines)
4) Immediate Actions
5) Prevention"
    );
    messages.push(("system".to_string(), system));
    let mut user = String::new();
    user.push_str("Analyze these logs:\n\n");
    user.push_str(log_text);
    messages.push((String::from("user"), user));
    messages

}