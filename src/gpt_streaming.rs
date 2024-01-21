use futures_util::StreamExt;
use std::env;
use tokio::sync::mpsc::{self, Sender};

use chatgpt::{client::ChatGPT, types::ResponseChunk};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct PartialEvaluationPayload {
    relevance: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvaluationResult {
    relevance: u8,
    keywords: Vec<String>,
    reason: String,
}

async fn query_and_cache(
    request_id: String,
    client_query: String,
    partial_send_channel: mpsc::Sender<u8>,
    evaluation_send_channel: Sender<(String, EvaluationResult)>,
) -> Result<(), chatgpt::err::Error> {
    let oai_token = env::var("OPENAI_API").expect("OPENAI_API must be set");
    let client = ChatGPT::new(oai_token).unwrap();

    let full_prompt = PRE_PROMPT.to_owned() + "\n" + &client_query;
    let mut stream = client.send_message_streaming(full_prompt).await?;

    let mut full_response: String = String::from("");
    let mut found = false;

    while let Some(chunck) = stream.next().await {
        if let ResponseChunk::Content {
            ref delta,
            response_index: _,
        } = chunck
        {
            if let Ok(parse) = delta.parse::<u8>() {
                if !found {
                    partial_send_channel.send(parse).await.unwrap();
                    found = true;
                }
            };

            full_response += &delta;
        };

        if let ResponseChunk::CloseResponse { response_index: _ } = chunck {
            break;
        }
    }

    let ev_result: EvaluationResult = serde_json::from_str(&full_response).unwrap();
    evaluation_send_channel.send((request_id, ev_result)).await;

    Ok(())
}

pub async fn stream_gpt(
    client_query: String,
    request_id: String,
    coordinator_channel: Sender<(String, EvaluationResult)>,
) -> Result<PartialEvaluationPayload, chatgpt::err::Error> {
    let (sender, mut receiver) = mpsc::channel::<u8>(1);

    tokio::spawn(async {
        query_and_cache(request_id, client_query, sender, coordinator_channel)
            .await
            .unwrap();
    });

    if let Some(relevance) = receiver.recv().await {
        receiver.close();
        Ok(PartialEvaluationPayload { relevance })
    } else {
        panic!();
    }

    // println!("Relevance: {relevance}");

    // Ok(PartialEvaluationPayload { relevance })
}
