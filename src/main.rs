use std::{cmp::Reverse, collections::HashMap};

// This program is generated by ChatGPT too
// developers shoot themselves in the foot, now ChatGPT
// shoot itself in the foot
fn generate_dictionary(sentences: &[&str]) -> HashMap<String, String> {
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    // Count the number of occurrences of each word in the sentences
    for sentence in sentences {
        for word in sentence.split_whitespace() {
            *word_counts.entry(word.to_lowercase()).or_default() += 1;
        }
    }

    // Sort the words by frequency
    let mut words: Vec<_> = word_counts.keys().cloned().collect();
    words.sort_by_key(|word| Reverse(word_counts[word]));

    // Assign each word a token
    let mut dictionary = HashMap::new();
    let mut token = "a".to_string();
    for word in words {
        if word_counts[&word] > 0 {
            dictionary.insert(word, token.clone());

            // Update the next token
            let last_char = token.chars().last().unwrap();
            if last_char == 'z' {
                token.pop();
                token.push((last_char as u8 + 1) as char);
                token.push('a');
            } else {
                token.pop();
                token.push((last_char as u8 + 1) as char);
            }
        }
    }

    dictionary
}

fn translate_sentence(sentence: &str, dictionary: &HashMap<String, String>) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            dictionary
                .get(word.to_lowercase().as_str())
                .unwrap_or(&word.to_string())
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let sentences = vec!["Generate a program that prints The quick brown fox jumps over the lazy dog in rust", "Generate a program that prints Hello World in rust", "Write a rust program that outputs 'The quick brown fox jumps over the lazy dog'", "Create a rust code that displays 'The quick brown fox jumps over the lazy dog'", "Build a rust program that prints 'The quick brown fox jumps over the lazy dog'", "Compose a rust script that prints the sentence 'The quick brown fox jumps over the lazy dog'", "Design a rust program that says 'The quick brown fox jumps over the lazy dog' on the console", "Make a rust code that shows 'The quick brown fox jumps over the lazy dog' on the screen", "Develop a rust program that prints 'The quick brown fox jumps over the lazy dog' on the output"];
    let dictionary = generate_dictionary(&sentences);

    let prompt = format!("
        I will now talk only with a token generated sentence based on the following dictionnary:

       {dictionary:?}

        Every time I write a new prompt use the dictionary to translate the token based sentence into a prompt that you can understand. Then act as if it was the original prompt and execute what the prompt says.
    ");

    println!("{prompt:?}");

    for sentence in sentences {
        println!(
            "{} => {}",
            sentence,
            translate_sentence(sentence, &dictionary)
        );
    }
}
