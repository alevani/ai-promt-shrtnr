# PRE-READ DISCLAIMER
This is a version 0.1.

# AI Prompt Shrtnr
With the emergence of AI, businesses that relies on heavy trained natural language processing models are flourishing. The most used business model seems to be "$ per token used" when sending a prompt to an natural language processing API.


This tools serves as a middleman dictionnary injection that can be used to reduce the length of your prompt, hence saving you money.


For instance, the following prompt
```
Write a short rust program that takes two list of number as input, and computes the GCD of the two list
```

which is 103 characters long, could be shorten to

```
O A V B J C W P Q R X Y Z AA AB  AC R  P Q
```

42 characters in length, saving half the cost of the original prompt.


This tool is useful if your API performs many of the same prompts, as the first heading prompt that contains the dictionnary can be quite expensive.

## TODOS
- Maybe we can add a version that shortens only the words by removing the vowels. this way it could be any sentence very easily, and the heading prompt could be "Consider the following prompt, where I removed all the vowels..."
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

    {"the": "", "a": "A", "rust": "B", "that": "C", "quick": "D", "brown": "E", "fox": "F", "jumps": "G", "over": "H", "lazy": "I", "program": "J", "'the": "K", "dog'": "L", "prints": "M", "on": "N", "write": "O", "two": "P", "list": "Q", "of": "R", "generate": "S", "in": "T", "code": "U", "short": "V", "takes": "W", "number": "X", "as": "Y", "input,": "Z", "and": "AA", "computes": "AB", "gcd": "AC", "dog": "AD", "hello": "AE", "world": "AF", "outputs": "AG", "create": "AH", "displays": "AI", "build": "AJ", "compose": "AK", "script": "AL", "sentence": "AM", "design": "AN", "says": "AO", "console": "AP", "make": "AQ", "shows": "AR", "screen": "AS", "develop": "AT", "output": "AU"}

Every time I write a new prompt, use the dictionary to translate the token based sentence into a prompt that you can understand. Then act as if it was the original prompt and execute what the prompt says.

Write a short rust program that takes two list of number as input, and computes the GCD of the two list => O A V B J C W P Q R X Y Z AA AB  AC R  P Q
Generate a program that prints The quick brown fox jumps over the lazy dog in rust => S A J C M  D E F G H  I AD T B
Generate a program that prints Hello World in rust => S A J C M AE AF T B
Write a rust program that outputs 'The quick brown fox jumps over the lazy dog' => O A B J C AG K D E F G H  I L
Create a rust code that displays 'The quick brown fox jumps over the lazy dog' => AH A B U C AI K D E F G H  I L
Build a rust program that prints 'The quick brown fox jumps over the lazy dog' => AJ A B J C M K D E F G H  I L
Compose a rust script that prints the sentence 'The quick brown fox jumps over the lazy dog' => AK A B AL C M  AM K D E F G H  I L
Design a rust program that says 'The quick brown fox jumps over the lazy dog' on the console => AN A B J C AO K D E F G H  I L N  AP
Make a rust code that shows 'The quick brown fox jumps over the lazy dog' on the screen => AQ A B U C AR K D E F G H  I L N  AS
Develop a rust program that prints 'The quick brown fox jumps over the lazy dog' on the output => AT A B J C M K D E F G H  I L N  AU
```

## SEE IT WORKING
![](./demo_files/Screenshot%202023-02-08%20at%2016.23.47.png)
