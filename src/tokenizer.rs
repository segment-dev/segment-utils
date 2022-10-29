use anyhow::Result;

pub fn tokenize(input: String) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut open_quotes = false;
    for char in input.chars() {
        if char == ' ' {
            if open_quotes {
                token.push(char);
            } else if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        } else if char == '"' {
            if open_quotes {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                open_quotes = false;
            } else {
                open_quotes = true;
            }
        } else {
            token.push(char);
        }
    }

    if !token.is_empty() {
        tokens.push(token)
    }

    if open_quotes {
        return Err(anyhow::anyhow!("malformed input received for tokenization"));
    }

    Ok(tokens)
}
