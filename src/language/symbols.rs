#![allow(dead_code)]

// Check if a string is a valid name
pub fn is_valid_name(s: &String) -> bool
{
    fn is_valid_char(c: char) -> bool
    {
        match c
        {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            '_' | '-' | '?' | '$' => true,

            _ => false,
        }
    }
    fn is_valid_first_char(c: char) -> bool
    {
        match c
        {
            '0'..='9' => false,
            '_' | '-' | '?' | '$' => false,
            c => is_valid_char(c),
        }
    }

    let mut last_char = ' ';
    for (i, c) in s.chars().enumerate()
    {
        if last_char == '_' && c == '_'
        {
            return false;
        }
        else if !is_valid_char(c)
        {
            return false;
        }
        else if i == 0 && !is_valid_first_char(c)
        {
            return false;
        }

        last_char = c;
    }

    return true;
}

// Convert a valid name to a valid C name
pub fn convert_to_c_safe(s: &String) -> String
{
    let mut result = s.clone();
    result = result
        .replace("-", "__")
        .replace("?", "_question_")
        .replace("$", "_dollar_")
        .replace("/", "__");
    return result;
}

// Defines constant strings, get_hash_set function, and contains (pattern match) function
macro_rules! symbols {
    [$($value:expr => $name:ident),*] => {
        $(pub const $name: &str = $value;)*

        use std::collections::HashSet;
        pub fn get_hash_set() -> HashSet<String>
        {
            let mut result: HashSet<String> = HashSet::new();
            $(result.insert($name.to_owned());)*
            return result;
        }

        pub fn contains(s: &String) -> bool
        {
            match s.as_str()
            {
                $($name => true,)*
                _ => false,
            }
        }
    };
}

pub mod operators
{
    symbols![
        "not" => NOT,
        "ref" => REFERENCE,
        "mut" => MUTABLE_REFERENCE,
        "@" => DEREFERENCE,
        "." => ACCESS,
        "=" => ASSIGN,
        "+" => PLUS,
        "-" => MINUS,
        "*" => TIMES,
        "/" => DIVIDE,
        "%" => MODULO,
        "^" => POW,
        "==" => EQUAL,
        "=/=" => NOT_EQUAL,
        "<" => LESS,
        ">" => GREATER,
        "<=" => LESS_EQUAL,
        ">=" => GREATER_EQUAL,
        "and" => AND,
        "or" => OR,
        "xor" => XOR
    ];

    pub const ACCESS_CHAR: char = '.';

    pub fn is_binary(s: &String) -> bool
    {
        match s.as_str()
        {
            // ACCESS | ASSIGN | MINUS | TIMES | DIVIDE | MODULO | POW |
            PLUS | EQUAL | NOT_EQUAL | LESS | GREATER | LESS_EQUAL | GREATER_EQUAL | AND | OR
            | XOR => true,
            _ => false,
        }
    }

    pub fn is_unary(s: &String) -> bool
    {
        match s.as_str()
        {
            // NOT | MINUS | REFERENCE | MUTABLE_REFERENCE | DEREFERENCE => true,
            REFERENCE | MUTABLE_REFERENCE | DEREFERENCE => true,
            _ => false,
        }
    }
}

pub mod keywords
{
    symbols![
        "fn" => FUNCTION,
        "->" => ARROW,
        "let" => BINDING,
        "struct" => STRUCT,
        "public" => PUBLIC,
        "private" => PRIVATE,
        "if" => IF,
        "then" => THEN,
        "else" => ELSE,
        "#" => COMMENT
    ];

    pub const LINE_COMMENT_CHAR: char = '#';

    pub const BLOCK_COMMENT_START_CHAR: char = '<';
    pub const BLOCK_COMMENT_END_CHAR: char = '>';
    pub const BLOCK_COMMENT_CHAR_COUNT: usize = 3;
}

pub mod constants
{
    symbols![
        "true" => TRUE,
        "false" => FALSE,
        "self" => SELF
    ];
}

pub mod delimiters
{
    pub const STRING: &str = "\"";
}

pub mod primitive_data_types
{
    symbols![
        "int" => INTEGER,
        "bool" => BOOLEAN,
        "float" => FLOAT,
        "void" => VOID,
        "tag" => SHORTSTRING,
        "string" => LONGSTRING
    ];
}

// Get a hash set containing all symbols
use std::collections::HashSet;
pub fn get_hash_set() -> HashSet<String>
{
    let mut result: HashSet<String> = HashSet::new();

    for s in operators::get_hash_set().drain()
    {
        result.insert(s);
    }
    for s in keywords::get_hash_set().drain()
    {
        result.insert(s);
    }
    for s in constants::get_hash_set().drain()
    {
        result.insert(s);
    }
    for s in primitive_data_types::get_hash_set().drain()
    {
        result.insert(s);
    }

    return result;
}
