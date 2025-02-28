use deepseek_api_client::*;
use futures_util::StreamExt;
use tokio::runtime::Runtime;
use std::io::{self, Write};

use lazy_static::lazy_static;
lazy_static! {
    static ref DEEPSEEK_API_KEY: String = std::env::var("DEEPSEEK_API_KEY").unwrap();
}
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    //call API in async function
    let messages = vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant".to_owned(),
            },
            Message {
                role: "user".to_owned(),
                content: "Write Hello world in rust".to_owned(),
            },
        ];
        let mut llm = chat_completion(&DEEPSEEK_API_KEY);
        let res = llm(messages.clone()).await?;
        let res_text = get_response_text(&res, 0);
        println!("{:?}",res_text);
    
    //call API in Stream
    let mut stream_llm = chat_completion_stream(&DEEPSEEK_API_KEY);        
    let dt = stream_llm(messages.clone());
    let res = dt.await?;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item.unwrap();
        let s = match std::str::from_utf8(&item) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let data = string_to_ChatResponses(s);
        let res = get_response_text(&data, 0).unwrap_or("".to_owned());

        print!("{}", res);
        
    }
     

    Ok(())    
}

fn call_api_in_sync_function(){
    //call API in sync function
    let messages = vec![
            Message {
                role: "system".to_owned(),
                content: "You are a helpful assistant".to_owned(),
            },
            Message {
                role: "user".to_owned(),
                content: "Write Hello world in rust".to_owned(),
            },
        ];
    let mut sync_llm = chat_completion_sync(&DEEPSEEK_API_KEY);
    let res = sync_llm(messages);
    let res_text = get_response_text(&res.unwrap(), 0);
    println!("{:?}",res_text);
}
