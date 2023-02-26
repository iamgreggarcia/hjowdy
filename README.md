# Hjowdy

A Rust-based HTTP server and API wrapper that provides a single API endpoint for interacting with the OpenAI GPT-3 API. The server responds to POST requests with a JSON object containng
a prompt and returns the generated text from OpenAI's GPT-3 model (e.g., text-davinci-003) or a fine tuning
model of your own.

This server can be used for a starting point for building more complex applications that incorporate OpenAI's API.

## Installation

1. Clone the repository: `git clone https://github.com/iamgreggarcia/hjowdy.git`
2. Navigate to the project directory: `cd hjowdy`
3. Set your OpenAI API key as an environment variable named `OPENAI_API_KEY`.
4. Run the project with `cargo run`.
5. The web server will start on `http://127.0.0.1:8080`.

## Usage

To generate a response, make a GET request to `http://127.0.0.1:8080/Response/{prompt}`, where `{prompt}` is the prompt string for which you want a response.

Example usage:

```shell
curl http://127.0.0.1:8080/Response/Hello,%20how%20are%20you%20today?
```

This will return a generated response from the OpenAI API based on the prompt "Hello, how are you today?".

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

