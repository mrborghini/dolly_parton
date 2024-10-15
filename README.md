# Dolly parton

This discord bot uses Ollama and has some commands you can give it.

## Prerequisites
- [Cargo](https://rustup.rs/) or [Docker](https://www.docker.com/)
- [Ollama](https://ollama.com/)


## How to setup

1. Create a .env file by running.

```
cp .env.example .env
```
2. Put in your credentials

3. Setup a system message.
```
cp system_message_example.txt system_message.txt
```
4. Edit the system_message.txt to your liking

5. Now build and run the bot by running
```
cargo run --release
```
6. **(optional)** You can also use Docker by running

```
docker compose up
```

## .env configuration

This is a basic config

```dosini
DISCORD_TOKEN=yourtoken
GUILD_ID=84343943
OLLAMA_URL=http://localhost:11434
OLLAMA_MODEL=llama3.2
RESPOND_TO_ALL_MESSAGES=false
RESPONDS_TO=dolly,=gm,goodmorning
MAX_STORED_MESSAGES=6
LOGGER_DEBUG=false
WRITE_LOGS=false
```

* `DISCORD_TOKEN` is the token for your bot
* `GUILD_ID` is the id of your discord server
* `OLLAMA_URL` is the url of the Ollama server
* `OLLAMA_MODEL` is the model for the Ollama server
* `OLLAMA_MODEL` is the model for the Ollama server
* `RESPOND_TO_ALL_MESSAGES` whether the bot should respond to all messages it receives with Ollama
* `RESPONDS_TO` All the things the bot will respond to. It's comma seperated. By default it checks if the string is in the message, but for exact matches use `=` infront of the string you want to match.
* `MAX_STORED_MESSAGES` Is the max amount of messages that get stored in `out_data`. With the model `llama3.2` I notice that after 7 messages the quality drops, so setting this to 6 is a good balance and prevent people from spamming too many messages. To remove the limit set it to `0`.
* `WRITE_LOGS` It just creates a log file in `out_data`
* `LOGGER_DEBUG` It shows more debug information in the terminal and `out_data` if you have `WRITE_lOGS` set to true