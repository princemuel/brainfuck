use core::iter::Peekable;
use core::str::Chars;

/// Brainfuck lexer that converts source code into an AST
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from source code
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self { chars: source.chars().peekable(), position: Position::new() }
    }
}

/// Represents a position in the source code for error reporting
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    #[must_use]
    pub const fn new() -> Self { Self { row: 1, col: 1 } }

    #[inline]
    pub const fn advance(&mut self, value: char) {
        if value == '\n' {
            self.row += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

impl Default for Position {
    #[inline]
    fn default() -> Self { Self::new() }
}

#[expect(clippy::exhaustive_enums, reason = "this is already specified")]
/// Basic Brainfuck commands (excluding loop constructs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Right,     // >
    Left,      // <
    Increment, // +
    Decrement, // -
    Output,    // .
    Input,     // ,
}

impl Command {
    /// Parse a single command character
    const fn parse(value: char) -> Option<Self> {
        match value {
            '>' => Some(Self::Right),
            '<' => Some(Self::Left),
            '+' => Some(Self::Increment),
            '-' => Some(Self::Decrement),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            _ => None,
        }
    }
}

#[expect(clippy::exhaustive_enums, reason = "this is already specified")]
/// Represents a single Brainfuck token/command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    /// Increment data pointer `>`.
    Right,
    /// Decrement data pointer `<`.
    Left,
    /// Increment current cell `+`.
    Increment,
    /// Decrement current cell `-`.
    Decrement,
    /// Output current cell as ASCII `.`.
    Output,
    /// Input ASCII character to current cell `,`.
    Input,
    /// Start of loop `[`.
    LoopStart,
    /// End of loop `]`.
    LoopEnd,
    /// End of file/input.
    Eof,
}

impl Token {
    /// Convert a character to its corresponding Brainfuck token.
    ///
    /// Returns Some(Token) for valid Brainfuck commands, None for
    /// comments/ignored characters.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Token::Right),
            '<' => Some(Token::Left),
            '+' => Some(Token::Increment),
            '-' => Some(Token::Decrement),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            _ => None, // All other characters are ignored (comments)
        }
    }

    /// Get the character representation of this token.
    #[inline]
    #[must_use]
    pub const fn as_char(&self) -> char {
        match *self {
            Token::Left => '<',
            Token::Right => '>',
            Token::Increment => '+',
            Token::Decrement => '-',
            Token::Input => ',',
            Token::Output => '.',
            Token::LoopStart => '[',
            Token::LoopEnd => ']',
            Token::Eof => '\0',
        }
    }
}

impl core::fmt::Display for Token {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_commands() {}
}
