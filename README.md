# Hjowdy

Hjowdy is a Rust application that allows you to interact with the OpenAI API to generate chat responses and text completions. This project is a simple wrapper API implementation, with support for text completion prompts and chat completion.

## Getting started
Before you start using hjowdy, make sure you have your OpenAI API key handy: [OpenAI](https://platform.openai.com/)

### Running hjowdy locally
To run hjowdy locally, you'll need to have Rust installed. Once you've installed Rust, clone the hjowdy repository and navigate to the directory:

```bash
git clone https://github.com/<your_github_username>/hjowdy.git
cd hjowdy

```

Next, set an environment variable `OPENAI_API_KEY`:

```bash
export OPENAI_API_KEY=<your_openai_api_key>
```


Finally, run the project using Cargo:

```bash
cargo run
```

Hjowdy will be running on `http://localhost:8080`.

## Using the API

### Text Completion Prompt

Uses the `POST https://api.openai.com/v1/completions` OpenAI API found [here](https://platform.openai.com/docs/api-reference/completions)
> Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position.


To generate text completion prompts, make a POST request to http://localhost:8080/text_completion_prompt with a JSON body that contains the text 

```bash
 curl -X POST \
  http://localhost:8080/text_completion_prompt \
  -H 'Content-Type: application/json' \
  -d '{"prompt":"I want to print the rust logo in ASCII"}'


```

Response:

```bash
{"id":"cmpl-6qTNWvv4JS7v9nJxsofGmONtDzJM0",
"object":"text_completion","created":1677964006,"model":"text-davinci-003","choices":
[{"text":"\n\n __   __   /\\   _____  \\ \\      / /  _____|\n \\ \\ / /  /  \\ / ____/  |\\ \\  /\\  / /  |  __\n  \\ V /  / /\\ \\\\___ \\    \\ \\/  \\/ /| | |_ |\n   > <  / ____ \\ ___) |    \\  /\\  / | |__| |\n  / . \\/_/    \\/_____/      \\/  \\/   \\_____|\n /_/\n \n ██╗   ██╗ ███████╗██╗███╗   ██╗ ██████╗\n ██║   ██║ ██╔════╝██║████╗  ██║██╔════╝\n ██║   ██║ █████╗  ██║██╔██╗ ██║██║  ███╗\n ██║   ██║ ██╔══╝  ██║██║╚██╗██║██║   ██║\n ╚██████╔╝ ███████╗██║██║ ╚████║╚","index":0,"logprobs":null,"finish_reason":"length"}],"usage":{"prompt_tokens":9,"completion_tokens":300,"total_tokens":309}}

``` 
(clearly not its best work!)

### Chat Completion

Uses the `POST https://api.openai.com/v1/chat/completions` found [here](https://platform.openai.com/docs/api-reference/chat/create)

To generate chat completions, make a POST request to http://localhost:8080/chat with a JSON body that contains an array of messages:

```bash
curl -X POST \
  http://localhost:8080/chat \
  -H 'Content-Type: application/json' \
  -d '{
        "messages": [
            {
                "role": "system",
                "content": "Hej. You are a darn good chatbot who is very
 knowledgeable about all things Rust. You also love memes."
            },
            {
                "role": "user",
                "content": "Hey, explain Rust lifetimes to me in 7 words."
            }
        ]
    }'

```

Response:

```bash
{
  "id": "chatcmpl-6qTIYKEnNfs14a5TuhQm4FDG1OmuU",
  "object": "chat.completion",
  "created": 1677963698,
  "model": "gpt-3.5-turbo-0301",
  "usage": {
    "prompt_tokens": 49,
    "completion_tokens": 11,
    "total_tokens": 60
  },
  "choices": [
    {
      "message": {
        "role": "assistant",
        "content": "Borrow checker ensures safe code ownership relations."
      },
      "finish_reason": "stop",
      "index": 0
    }
  ]
}
```

## TODO
* Integration tests: Write integration tests to ensure hjowdy is working as expected.
* CI/CD with GitHub Actions: Add continuous integration and deployment using GitHub Actions.
* In-memory data structure for chat history: Store the chat history in an in-memory data structure for quick prototyping.
* Switch to a more scalable and persistent storage solution: Once hjowdy is stable, consider switching to a more scalable and persistent storage solution like a database.
* Create a simple client side application: Eventually create a simple client side application to interact with the API.
* Parse response from OpenAI API before sending back to hjowdy API caller
