# hjowdy

A simple Rust API that uses the Actix web framework and the Reqwest HTTP client library to call the OpenAI API and generate text completions based on prompts. The project defines an endpoint `/Response/{request_prompt}` that takes a prompt string as a parameter and returns a generated response from the OpenAI API.

## Prerequisites

- Rust
- OpenAI API key (to access the OpenAI API)

## Installation

1. Clone the repository: `git clone https://github.com/your-username/hjowdy.git`
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

