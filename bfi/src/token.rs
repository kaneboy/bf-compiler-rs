use anyhow::Result;

/// 合法的源代码字符。
#[derive(Debug, Copy, Clone)]
pub struct Token(pub char);

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '+' => Ok(Token('+')),
            '-' => Ok(Token('-')),
            '>' => Ok(Token('>')),
            '<' => Ok(Token('<')),
            '[' => Ok(Token('[')),
            ']' => Ok(Token(']')),
            '.' => Ok(Token('.')),
            ',' => Ok(Token(',')),
            _   => anyhow::bail!("invalid token."),
        }
    }
}

impl From<Token> for char {
    fn from(token: Token) -> Self {
        token.0
    }
}
