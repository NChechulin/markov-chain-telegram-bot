# Markov Chain telegram bot

This is a small telegram bot built in Rust using [telegram-bot](https://github.com/telegram-rs/telegram-bot) and [markov](https://github.com/aatxe/markov).
I also used [Blog Authorship Corpus](https://www.kaggle.com/rtatman/blog-authorship-corpus) to provide a sample feed file.

## Setup and run

1. Ensure that you have rust and `cargo` installed
2. Clone the repo: `git clone https://github.com/NChechulin/markov-chain-telegram-bot.git`
3. Register new bot and insert it's token in `src/main.rs:26`.
4. Run `cargo run`

If you want to change/add files which are feeded to the bot, just drop your text files in `feed_files/`.
The bot will automatically feet all the files inside that folder.
