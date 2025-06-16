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
# Discord Bot Configuration
DISCORD_TOKEN=yourtoken             # Token for your Discord bot
GUILD_ID=84343943                   # ID of your Discord server
ALLOW_CHANGING_SYSTEM_PROMPT=false  # This allows a user to change the system prompt inside of system_message.txt

# AI providers
OPENAI_TOKEN=                       # Optional leave empty like this to not use this feature.
OPENAI_MODEL=gpt-4o                 # Change the model is you set the openai token (optional)

COHERE_TOKEN=                       # Optional leave empty like this to not use this feature.
COHERE_MODEL=command-r-plus-08-2024 # Change the model is you set the cohere token (optional)

# Ollama Server Configuration
OLLAMA_URL=http://localhost:11434   # URL of the Ollama server
OLLAMA_MODEL=llama3.2               # Model to use on the Ollama server
PRIORTIZE_OLLAMA=true               # If you want to use Ollama as a base, but if it's not available it will use the others if you set the one of the tokens
NUM_CTX=2048                        # If you want ollama to remember more stuff you can change this to a higher value

# Bot Response Behavior
RESPOND_TO_ALL_MESSAGES=false       # Whether the bot should respond to all messages (true/false)
RESPONDS_TO=dolly,=gm,goodmorning   # Comma-separated triggers (prefix with = for exact match)

# Message Handling
MAX_STORED_MESSAGES=6               # Max stored messages (0 = no limit)

# Logging Configuration
LOGGER_DEBUG=false                  # Enable detailed logs if WRITE_LOGS is true
WRITE_LOGS=false                    # Enable writing logs to 'out_data'
```

- `DISCORD_TOKEN` is the token for your bot
- `GUILD_ID` is the id of your discord server
- `ALLOW_CHANGING_SYSTEM_PROMPT` If set to `true` the `/change_system_prompt` command will work.
- `OPENAI_TOKEN` is the optional token if you want to use OpenAI instead of
  Ollama
- `OPENAI_MODEL` is the optional model if you want to use OpenAI instead of
  Ollama
- `COHERE_TOKEN` is the optional token if you want to use Cohere instead of
  Ollama
- `COHERE_TOKEN` is the optional model if you want to use Cohere instead of
  Ollama
- `OLLAMA_URL` is the url of the Ollama server
- `OLLAMA_MODEL` is the model for the Ollama server
- `OLLAMA_MODEL` is the model for the Ollama server
- `NUM_CTX` is the amount of tokens a message array can have.
- `PRIORTIZE_OLLAMA` It will use Ollama over other providers if set to true.
- `RESPOND_TO_ALL_MESSAGES` whether the bot should respond to all messages it
  receives with Ollama
- `RESPONDS_TO` All the things the bot will respond to. It's comma seperated. By
  default it checks if the string is in the message, but for exact matches use
  `=` infront of the string you want to match.
- `MAX_STORED_MESSAGES` Is the max amount of messages that get stored in
  `out_data`. With the model `llama3.2` I notice that after 7 messages the
  quality drops, so setting this to 6 is a good balance and prevent people from
  spamming too many messages. To remove the limit set it to `0`.
- `WRITE_LOGS` It just creates a log file in `out_data`
- `LOGGER_DEBUG` It shows more debug information in the terminal and `out_data`
  if you have `WRITE_lOGS` set to true
