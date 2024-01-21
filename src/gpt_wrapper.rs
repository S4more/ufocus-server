use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct OAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct OAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Usage {}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvaluationResult {
    relevance: u8,
    keywords: Vec<String>,
    reason: String,
}

const URI: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";

const PRE_PROMPT: &str = r#"You're a productivity assistant browser extension. You will be prompted with the text of the webpage the user is accessing. You will rate the relevance of the page to that user based on a user description. 

Your output will be made in the following json format:
{
"relevance": int, # from 0 to 10
"reason": str, # a 150 word max reason for the relevance. Don't use the 1st person.
"keywords": list[str] # a list of important words that matches or doesn't match the relevance criteria
}

the relevance is mainly determined by two factors:
1.  Is it related to the user's goal?
2. Is it related to the goal's field of knowledge or profession?

user-description: Junior Software Developer at LogMeIn. Working on Terraform to update his AWS stack.

page-text:"#;

pub async fn query_gpt(query: String) -> Result<EvaluationResult, Box<dyn std::error::Error>> {
    let oai_token = env::var("OPENAI_API").expect("OPENAI_API must be set");
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", oai_token).parse().unwrap(),
    );

    let prompt_message: Message = Message {
        role: String::from("system"),
        content: String::from(PRE_PROMPT),
    };

    let req = OAIRequest {
        model: String::from(MODEL),
        messages: vec![
            prompt_message,
            Message {
                role: String::from("user"),
                content: query,
            },
        ],
    };

    let res = client
        .post(URI)
        .headers(headers)
        .json(&req)
        .send()
        .await?
        .json::<OAIResponse>()
        .await?;

    // extract out the last index of the choices vector and get the message
    let message = res
        .choices
        .last()
        .ok_or("No choices returned")?
        .message
        .content
        .clone();

    match serde_json::from_str::<EvaluationResult>(&message) {
        Ok(value) => Ok(value),
        Err(e) => Err(Box::new(e)),
    }
}
