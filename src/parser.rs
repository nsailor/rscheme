
#[derive(Debug)]
#[derive(Clone)]
pub enum PrimitiveToken {
    LeftParen,
    RightParen,
    Word(String),
    StringLiteral(String),
    NumericLiteral(f64),
    Quote,
}

fn split_word(accum: &mut String, tokens: &mut Vec<PrimitiveToken>) {
    if !accum.is_empty() {
        match accum.to_string().parse::<f64>() {
            Ok(v) => tokens.push(PrimitiveToken::NumericLiteral(v)),
            Err(_) => tokens.push(PrimitiveToken::Word(accum.to_string())),
        };
        *accum = String::new();
    }
}

pub fn parse_primitives(code: &String) -> Vec<PrimitiveToken> {
    let mut tokens: Vec<PrimitiveToken> = Vec::new();
    // Parser state machine
    let mut word_accumulator: String = String::new();
    let mut in_comment = false;
    let mut in_string = false;
    for c in code.chars() {
        if in_comment {
            if c == '\n' {
                in_comment = false;
            }
        } else if in_string {
            if c == '\"' {
                in_string = false;
                tokens.push(PrimitiveToken::StringLiteral(word_accumulator));
                word_accumulator = String::new();
            } else {
                word_accumulator.push(c);
            }
        } else {
            match c {
                '(' | '[' => {
                    split_word(&mut word_accumulator, &mut tokens);
                    tokens.push(PrimitiveToken::LeftParen)
                }

                ')' | ']' => {
                    split_word(&mut word_accumulator, &mut tokens);
                    tokens.push(PrimitiveToken::RightParen)
                }

                ' ' | '\n' | '\t' | '\r' => split_word(&mut word_accumulator, &mut tokens),
                ';' => {
                    split_word(&mut word_accumulator, &mut tokens);
                    in_comment = true
                }

                '\"' => {
                    split_word(&mut word_accumulator, &mut tokens);
                    in_string = true
                }

                '\'' => {
                    split_word(&mut word_accumulator, &mut tokens);
                    tokens.push(PrimitiveToken::Quote)
                }
                _ => word_accumulator.push(c),
            }
        }
    }
    tokens
}
