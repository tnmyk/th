use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize, Debug)]
struct Part {
    text: String,
}

pub async fn get_command(api_key: &str, query: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-flash-lite:generateContent?key={}",
        api_key
    );

    let os = env::consts::OS;
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown shell".to_string());

    let system_prompt = format!(
        "You are an expert terminal assistant. 
The user is on the '{}' operating system using the '{}' shell.
Your goal is to provide exactly the terminal command the user is asking for.
RULES:
1. ONLY return the raw command string.
2. DO NOT include markdown formatting (like ```bash).
3. DO NOT include any explanations or conversational text.
4. If you absolutely cannot determine a command, return the string 'ERROR: <reason>'.",
        os, shell
    );

    let body = json!({
        "systemInstruction": {
            "parts": [{"text": system_prompt}]
        },
        "contents": [{
            "parts": [{"text": query}]
        }]
    });

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, text));
    }

    let gemini_res: GeminiResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let command = gemini_res
        .candidates
        .and_then(|mut c| c.pop())
        .and_then(|c| c.content.parts.into_iter().next())
        .map(|p| p.text.trim().to_string())
        .ok_or_else(|| "The API returned an unexpected or empty response.".to_string())?;

    if command.starts_with("ERROR:") {
        return Err(command);
    }

    Ok(command)
}
