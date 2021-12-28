
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tokens {
    // Symbol
    Plus,
    Lparen,
    Rparen,
    Colon,
    ColonColon,
    Semi,
    Rbrack,
    Lbrack,
    RCbrack,
    LCbrack,
    GT,
    LT,
    OArrowfunc,
    CArrowfunc,
    GTET,
    LTET,
    Equals,
    Minus,

    // Identifier
    Int,
    Float,
    Bool,
    True,
    False,
    Fn,
    String,
}

// match each type of token to an identifier so it can figure out what characters to look for
impl Tokens {
    pub fn value(&self) -> Option<&'static str> {
        match self {
            Self::Plus => Some("+"),
            Self::Lparen => Some("("),
            Self::Rparen => Some(")"),
            Self::Colon => Some(":"),
            Self::ColonColon => Some("::"),
            Self::Semi => Some(";"),
            Self::Rbrack => Some("]"),
            Self::Lbrack => Some("["),
            Self::RCbrack => Some("}"),
            Self::LCbrack => Some("{"),
            Self::GT => Some(">"),
            Self::LT => Some("<"),
            Self::OArrowfunc => Some("<("),
            Self::CArrowfunc => Some(")>"),
            Self::GTET => Some(">="),
            Self::LTET => Some("<="),
            Self::Equals => Some("="),
            Self::Minus => Some("-"),

            Self::Int => Some("int"),
            Self::Float => Some("float"),
            Self::Bool => Some("bool"),
            Self::True => Some("true"),
            Self::False => Some("false"),
            Self::Fn => Some("func"),
            Self::String => None,
        }
    }
}

// make it so it can print out
impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value().unwrap_or(""))
    }
}

fn tokenize(file: String) {
}
    
}
