use std::process;

use markov::Chain;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = Api::new("");
    let mut chain: Chain<String> = Chain::of_order(1);
    
    println!("Starting to feed the file...");

    match chain.feed_file("feed_files/blog_posts_en.txt") {
        Ok(_) => println!("File feeded, bot is starting..."),
        Err(_) => process::exit(1),
    }

    let mut stream = api.stream();
    
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Add message to markov chain
                chain.feed_str(data);

                let mut response = chain.generate_str_from_token(data);
                if response.len() == 0 {
                    response = chain.generate_str();
                }

                // Print received text message to stdout.
                println!("{} -> {}", data, response);

                // Answer message with "Hi".
                api.send(message.text_reply(response)).await?;
            }
        }
    }
    Ok(())
}
