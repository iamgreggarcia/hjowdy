# hjowdy

A Rust API for interacting with the OpenAI chat completion API.

## Overview

**hjowdy** is a simple Rust API designed to interact with the OpenAI chat API.  The API stores chat history and messages in a PostgreSQL database.

## Usage

### Setup Instructions

1. Clone the repository

```bash
git clone https://github.com/example/hjowdy.git
```

2. Install Rust and Cargo from the [official website](https://www.rust-lang.org/tools/install).

3. Follow the Database and Environment Variables section, provided below, to set up PostgreSQL and configure necessary environment variables.

4. Compile and run the API server

```bash
cargo run
```

### Database and Environment Variables 🗃️

**hjowdy** uses PostgreSQL as the database to store chat and message history. To set it up, follow these steps:

1. Install PostgreSQL on your system. [Download and installation instructions](https://www.postgresql.org/download/)

2. Ensure PostgreSQL is running.

3. Set up the required PostgreSQL credentials and OpenAI API key in the `.env` file:

Copy `.env.example` to `.env` and fill in the necessary variables

```
SERVER_ADDR=127.0.0.1:8080
PG.USER=<Your PostgreSQL username>
PG.PASSWORD=<Your PostgreSQL password>
PG.HOST=<Your PostgreSQL host>
PG.PORT=<Your PostgreSQL port>
PG.DBNAME=<Your PostgreSQL database name>
PG.POOL.MAX_SIZE=<Your PostgreSQL max pool size>
OPENAI_API_KEY=<Your OpenAI API key>
```
4. Run the `setup_database.sh` script to create the `chathistory` database and necessary tables:

```bash
chmod +x setup_database.sh
./setup_database.sh
```

## Examples

Refer to the Example CURLs section below for some examples of how to make requests to the API.

### Example CURLs

Here are some example CURL requests to help you get started:

1. Create a chat

```bash
curl -X POST "http://localhost:8080/create_chat/1"
```
2. Send a message to the chat

```bash
curl -X POST "http://localhost:8080/chat/1" \
-H "Content-Type: application/json" \
-d '{"messages": [{"role": "user", "content": "Hello!"}]}'
```

### API Specification

#### Endpoints

- `POST /create_chat/{app_user}` - Creates a new chat
- `GET /chats/{app_user}` - Retrieves all chats for the specified user
- `POST /chat/{chat_id}` - Sends a message and retrieves the chatbot response
- `GET /chats/{chat_id}/messages` - Retrieves all messages in a chat
- `PUT /update_chat_name` - Updates the chat name
- `DELETE /delete_chat/{chat_id}` - Deletes a chat

### Request and Response Examples

To interact with the API, send JSON payloads in the request body. For example, when calling the `/chat/{chat_id}` endpoint, the request body should look like this:

```json
{
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "Who won the world series in 2020?"
    }
  ]
}
```

The API will return a JSON object containing the chatbot's response. In the example above, the response might look like this:

```json
{
    "id": "chatcmpl-6p9XYPYSTTRi0xEviKjjilqrWU2Ve",
    "object": "chat.completion",
    "created": 1677649420,
    "model": "gpt-3.5-turbo-0301",
    "usage": {
        "prompt_tokens": 56,
        "completion_tokens": 31,
        "total_tokens": 87
    },
    "choices": [
        {
            "message": {
                "role": "assistant",
                "content": "The Los Angeles Dodgers won the World Series in 2020."
            },
            "finish_reason": "stop",
            "index": 0
        }
    ]
}
```

## Contributing

1. Fork the repository 🍴
2. Create a new branch with your feature or bugfix 🌿
3. Commit changes with descriptive commit messages 📝
4. Push your branch to the remote fork 🔌
5. Submit a pull request back to the original repository 🤲

For more information, check out the [OpenAI API documentation](https://platform.openai.com/docs/guides/chat).

Good luck! 🌄

```
