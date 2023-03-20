 Welcome to hjowdy, a Rust API for interacting with the OpenAI chat completion API. Follow the trail below to get started with some good ol' fashioned Rust programming. Yeehaw! 🤠 🐴

# 🌟 Setup Instructions
1. Clone the repository

`git clone https://github.com/example/hjowdy.git`

2. Install Rust

Make sure Rust and Cargo are installed on yer system. If they ain't, go ahead and rustle 'em up [here](https://www.rust-lang.org/tools/install).

3. Set up yer environment variables

Copy `.env.example` to `.env` and fill in the necessary variables, including your OpenAI API key.

4. Compile and run the API server

cargo run

# 🏃 Example CURL Requests
Here are some example CURL requests to help ya get started:

1. Create a chat

```python
curl -X POST "http://localhost:8080/create_chat/1"
```
2. Send a message to the chat

```json
curl -X POST "http://localhost:8080/chat/1" \
-H "Content-Type: application/json" \
-d '{"messages": [{"role": "user", "content": "Hello, partner!"}]}'
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
