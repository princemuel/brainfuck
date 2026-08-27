use core::fmt;

/// Represents a position in the source code for error reporting.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

/// Represents a single Brainfuck token/command, tagged with the
/// position at which it occurred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    pub token: T,
    pub position: Position,
}

/// A single Brainfuck command.
#[non_exhaustive]
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
}

impl TryFrom<char> for Token {
    type Error = ();

    /// Convert a character into its corresponding Brainfuck token.
    ///
    /// Returns `Err(())` for comments/ignored characters.
    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            '[' => Ok(Self::LoopStart),
            ']' => Ok(Self::LoopEnd),
            _ => Err(()),
        }
    }
}

impl From<Token> for char {
    #[inline]
    fn from(token: Token) -> Self {
        match token {
            Token::Left => '<',
            Token::Right => '>',
            Token::Increment => '+',
            Token::Decrement => '-',
            Token::Input => ',',
            Token::Output => '.',
            Token::LoopStart => '[',
            Token::LoopEnd => ']',
        }
    }
}

impl fmt::Display for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", char::from(*self)) }
}

/// Brainfuck lexer that turns source text into a stream of [`Spanned`] tokens.
///
/// Non-command characters (comments) are skipped transparently.
pub struct Lexer<'a> {
    chars: core::str::Chars<'a>,
    position: Position,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from source code.
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self { chars: source.chars(), position: Position::new() }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.chars.next()?;
            let position = self.position;
            self.position.advance(ch);

            if let Ok(token) = Token::try_from(ch) {
                return Some(Spanned { token, position });
            }
            // Not a command character; treat as a comment and continue.
        }
    }
}

impl core::iter::FusedIterator for Lexer<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_parse_simple_commands() {
        let expected: Vec<Token> = Lexer::new("+-><.,[]").map(|s| s.token).collect();
        assert_eq!(expected, vec![
            Token::Increment,
            Token::Decrement,
            Token::Right,
            Token::Left,
            Token::Output,
            Token::Input,
            Token::LoopStart,
            Token::LoopEnd,
        ]);
    }

    #[test]
    fn scenario_skips_comments_and_tracks_position() {
        let mut lexer = Lexer::new("+ hello\n-");
        let expected = lexer.next().unwrap();
        assert_eq!(expected.token, Token::Increment);
        assert_eq!(expected.position, Position { row: 1, col: 1 });

        let expected = lexer.next().unwrap();
        assert_eq!(expected.token, Token::Decrement);
        assert_eq!(expected.position, Position { row: 2, col: 1 });

        assert!(lexer.next().is_none());
    }
}
