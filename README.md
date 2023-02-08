# AI Prompt Shrtnr
As most natural language AI's business models are based on the number of token sent to their API, this tool saved you money by generated a token dictionnary in order to shorten the lenght of your prompts.

## Disclaimer
In a duty of sarcasm, most of the program has actually been written by an AI.

## How?
The tool simply analyses a set of given prompts by generating a word to frequency map, and attributing a short token to each word.

## RUN ME
```bash
cargo run
```

output:

```
I will now talk only with a token generated sentence based on the following dictionnary:

       {"rust": "c", "prints": "n", "hello": "{c", "build": "{i", "dog'": "l", "world": "v", "sentence": "y", "compose": "{f", "over": "i", "dog": "z", "that": "d", "brown": "h", "make": "u", "create": "t", "script": "{d", "output": "{g", "develop": "{j", "program": "m", "outputs": "x", "lazy": "e", "quick": "j", "generate": "p", "code": "q", "displays": "w", "a": "b", "jumps": "f", "write": "s", "design": "{a", "console": "{b", "the": "a", "screen": "{h", "'the": "k", "in": "r", "on": "o", "says": "{e", "shows": "{k", "fox": "g"}

Every time I write a new prompt use the dictionary to translate the token based sentence into a prompt that you can understand. Then act as if it was the original prompt and execute what the prompt says.

Generate a program that prints The quick brown fox jumps over the lazy dog in rust => p b m d n a j h g f i a e z r c
Generate a program that prints Hello World in rust => p b m d n {c v r c
Write a rust program that outputs 'The quick brown fox jumps over the lazy dog' => s b c m d x k j h g f i a e l
Create a rust code that displays 'The quick brown fox jumps over the lazy dog' => t b c q d w k j h g f i a e l
Build a rust program that prints 'The quick brown fox jumps over the lazy dog' => {i b c m d n k j h g f i a e l
Compose a rust script that prints the sentence 'The quick brown fox jumps over the lazy dog' => {f b c {d d n a y k j h g f i a e l
Design a rust program that says 'The quick brown fox jumps over the lazy dog' on the console => {a b c m d {e k j h g f i a e l o a {b
Make a rust code that shows 'The quick brown fox jumps over the lazy dog' on the screen => u b c q d {k k j h g f i a e l o a {h
Develop a rust program that prints 'The quick brown fox jumps over the lazy dog' on the output => {j b c m d n k j h g f i a e l o a {g
```
