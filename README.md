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