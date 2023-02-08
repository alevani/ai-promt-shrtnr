# AI Prompt Shrtnr
Most natural language AI's revenue is based on the number of tokens sent to their API, so this tool helps you save money by creating a token dictionary to shorten your prompts.

## TODOS
- Have the program to take a list of sentence
- Fix dictionnary problems
- Better prompt
- There are 1000 ways to improve the text analysis
- When translating to tokenized sentence, if a word does not exist, add it to the dictionnary

## DISCLAIMER
In a duty of irony, most of the program has actually been written by an AI.


This statement is intended to inform the reader that the information and materials provided through the program are intended for general informational purposes only, and should not be construed as a direct or indirect attack on any specific company. The use of the program is subject to the conditions that the user assumes full responsibility for any actions taken based on the information and materials provided. The program is provided "as is" without warranty of any kind, express or implied, and the user assumes all risks associated with its use.

## HOW?
The tool works by analyzing a set of input prompts to generate a map of word frequencies, then assigns a short token to each word.
The output is a prompt that contains the dictionnary to use, so that you can send it ahead of making your API calls.
You can also use the program to transalte a given sentence in to a token based sentence.
> Make sure that all the words from your sentence are in the dictionnary, otherwise they will be skipped
## RUN ME
```bash
cargo run
```

output:

```

    I will now talk only with a token generated sentence based on the following dictionnary:

    {"the": "", "a": "A", "that": "B", "rust": "C", "quick": "D", "brown": "E", "fox": "F", "jumps": "G", "over": "H", "lazy": "I", "'the": "J", "dog'": "K", "program": "L", "prints": "M", "on": "N", "generate": "O", "in": "P", "code": "Q", "dog": "R", "hello": "S", "world": "T", "write": "U", "outputs": "V", "create": "W", "displays": "X", "build": "Y", "compose": "Z", "script": "AA", "sentence": "AB", "design": "AC", "says": "AD", "console": "AE", "make": "AF", "shows": "AG", "screen": "AH", "develop": "AI", "output": "AJ"}

    Every time I write a new prompt, use the dictionary to translate the token based sentence into a prompt that you can understand. Then act as if it was the original prompt and execute what the prompt says.

Generate a program that prints The quick brown fox jumps over the lazy dog in rust => O A L B M  D E F G H  I R P C
Generate a program that prints Hello World in rust => O A L B M S T P C
Write a rust program that outputs 'The quick brown fox jumps over the lazy dog' => U A C L B V J D E F G H  I K
Create a rust code that displays 'The quick brown fox jumps over the lazy dog' => W A C Q B X J D E F G H  I K
Build a rust program that prints 'The quick brown fox jumps over the lazy dog' => Y A C L B M J D E F G H  I K
Compose a rust script that prints the sentence 'The quick brown fox jumps over the lazy dog' => Z A C AA B M  AB J D E F G H  I K
Design a rust program that says 'The quick brown fox jumps over the lazy dog' on the console => AC A C L B AD J D E F G H  I K N  AE
Make a rust code that shows 'The quick brown fox jumps over the lazy dog' on the screen => AF A C Q B AG J D E F G H  I K N  AH
Develop a rust program that prints 'The quick brown fox jumps over the lazy dog' on the output => AI A C L B M J D E F G H  I K N  AJ
```

## SEE IT WORKING
![](./demo_files/Screenshot%202023-02-08%20at%2016.23.47.png)
