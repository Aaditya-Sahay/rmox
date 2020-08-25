use crate::token::{Token, TokenType};
pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub parse_rules: Vec<ParseRule>,
}

impl Parser {
    pub fn new() -> Self {

        Self {
            current: Token {
                token_type: TokenType::NIL,
                start: 0,
                length: 0,
                line: 0,
                message: None,
            },
            previous: Token {
                token_type: TokenType::NIL,
                start: 0,
                length: 0,
                line: 0,
                message: None,
            },
            had_error: false,
            panic_mode: false,
            parse_rules: get_rule_array()
        }
    }
}
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Precedence {
    NONE,
    ASSIGNMENT, // =
    OR,         // or
    AND,        // and
    EQUALITY,   // == !=
    COMPARISON, // < > <= >=
    TERM,       // + -
    FACTOR,     // * /
    UNARY,      // ! -
    CALL,       // . ()
    PRIMARY,
}

#[derive(Copy, Clone, Debug)]
pub enum ParseFunctions {
    Grouping,   
    Unary,       
    Binary, 
    Number, 
    Null,
}
#[derive(Copy, Clone, Debug)]
pub struct ParseRule {
    pub prefix: ParseFunctions,
    pub infix: ParseFunctions,
    pub precedence: Precedence
}

impl ParseRule { 
    pub fn new(prefix: ParseFunctions, infix: ParseFunctions, precedence: Precedence) -> Self {
        Self { prefix, infix, precedence }
    }
    pub fn empty() -> Self {
        Self { 
            prefix: ParseFunctions::Null,    
            infix: ParseFunctions::Null,   
            precedence: Precedence::NONE 
        }
    }
}

fn get_rule_array() -> Vec<ParseRule> {
    let mut rules: Vec<ParseRule> = vec![ParseRule::empty(); 50];
    rules[TokenType::LEFT_PAREN as usize]    = ParseRule::new( ParseFunctions::Grouping,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::RIGHT_PAREN as usize]   = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::LEFT_BRACE as usize]    = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE ); 
    rules[TokenType::RIGHT_BRACE as usize]   = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::COMMA as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::DOT as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::MINUS as usize]         = ParseRule::new( ParseFunctions::Unary,    ParseFunctions::Binary, Precedence::TERM );
    rules[TokenType::PLUS as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Binary, Precedence::TERM );
    rules[TokenType::SEMICOLON as usize]     = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::SLASH as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Binary, Precedence::FACTOR);
    rules[TokenType::STAR as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Binary, Precedence::FACTOR);
    rules[TokenType::MOD as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Binary, Precedence::FACTOR);
    rules[TokenType::BANG as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::BANG_EQUAL as usize]    = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::EQUAL as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::EQUAL_EQUAL as usize]   = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::GREATER as usize]       = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::GREATER_EQUAL as usize] = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::LESS as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::LESS_EQUAL as usize]    = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::IDENTIFIER as usize]    = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::STRING as usize]        = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::NUMBER as usize]        = ParseRule::new( ParseFunctions::Number,   ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::AND as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::CLASS as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::ELSE as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::FALSE as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::FOR as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::FUN as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::IF as usize]            = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::NIL as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::OR as usize]            = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::PRINT as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::RETURN as usize]        = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::SUPER as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::THIS as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::TRUE as usize]          = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::VAR as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::WHILE as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::ERROR as usize]         = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );
    rules[TokenType::EOF as usize]           = ParseRule::new( ParseFunctions::Null,     ParseFunctions::Null,   Precedence::NONE );

    rules
}

