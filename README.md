 Welcome to **hjowdy**, a Rust API for interacting with the OpenAI chat completion API. Follow the trail below to get started with some good ol' fashioned Rust programming. Yeehaw! 🤠 🐴

## 🌟 Setup Instructions
1. Clone the repository

```bash
git clone https://github.com/example/hjowdy.git
```

2. Install Rust

Make sure Rust and Cargo are installed on yer system. If they ain't, go ahead and rustle 'em up [here](https://www.rust-lang.org/tools/install).

3. Set up yer environment variables

Copy `.env.example` to `.env` and fill in the necessary variables, including your OpenAI API key.

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


4. Compile and run the API server
```bash
cargo run
```

## 🏃 Example CURL Requests
Here are some example CURL requests to help ya get started:

1. Create a chat

```bash
curl -X POST "http://localhost:8080/create_chat/1"
```
2. Send a message to the chat

```bash
curl -X POST "http://localhost:8080/chat/1" \
-H "Content-Type: application/json" \
-d '{"messages": [{"role": "user", "content": "Hello, partner!"}]}'
```
## 📚 API Specification
### Endpoints

- `POST /create_chat/{app_user}` - Creates a new chat
- `GET /chats/{app_user}` - Retrieves all chats for the specified user
- `POST /chat/{chat_id}` - Sends a message and retrieves the chatbot response
- `GET /chats/{chat_id}/messages` - Retrieves all messages in a chat
- `PUT /update_chat_name` - Updates the chat name
- `DELETE /delete_chat/{chat_id}` - Deletes a chat

### Request and Response Examples

To interact with the API, send JSON payloads in the request body. For instance, when calling the `/chat/{chat_id}` endpoint, the request body should look like this:

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

# 🌱 Contributin'
1. If you feel like contributin', I'd be happier than a tornado in a trailer park. 🌪️
2. Fork the repository 🍴
3. Create a new branch with yer feature or bugfix 🌿
4. Commit changes (with some top-notch, dandy commit messages) 📝
5. Push yer branch to that remote fork 🔌
6. Submit a pull request back to the original repository 🤲

# 📚 Learnin' More
If you're hankerin' for more knowledge, check out the [OpenAI API documentation](https://platform.openai.com/docs/guides/chat).

Happy trails, partner! 🌄
