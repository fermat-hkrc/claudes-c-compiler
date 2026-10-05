use crate::common::encoding::decode_pua_byte;
use crate::common::source::Span;
use super::token::{Token, TokenKind};

/// C lexer that tokenizes source input with source locations.
pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    file_id: u32,
    gnu_extensions: bool,
}

impl Lexer {
    pub fn new(input: &str, file_id: u32) -> Self {
        Self {
            input: input.bytes().collect(),
            pos: 0,
            file_id,
            gnu_extensions: true,
        }
    }

    pub fn set_gnu_extensions(&mut self, enabled: bool) {
        self.gnu_extensions = enabled;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        // Estimate ~1 token per 5 bytes of source (typical for C code).
        let mut tokens = Vec::with_capacity(self.input.len() / 5);
        loop {
            let tok = self.next_token();
            let is_eof = tok.is_eof();
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.pos >= self.input.len() {
            return Token::new(TokenKind::Eof, Span::new(self.pos as u32, self.pos as u32, self.file_id));
        }

        let start = self.pos;
        let ch = self.input[self.pos];

        // Number literals
        if ch.is_ascii_digit() || (ch == b'.' && self.peek_next().is_some_and(|c| c.is_ascii_digit())) {
            return self.lex_number(start);
        }

        // String literals
        if ch == b'"' {
            return self.lex_string(start);
        }

        // Character literals
        if ch == b'\'' {
            return self.lex_char(start);
        }

        // Identifiers and keywords
        // GCC extension: '$' is allowed in identifiers (-fdollars-in-identifiers, on by default)
        if ch == b'_' || ch == b'$' || ch.is_ascii_alphabetic() {
            return self.lex_identifier(start);
        }

        // Punctuation and operators
        self.lex_punctuation(start)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }

            if self.pos >= self.input.len() {
                return;
            }

            // Skip GCC-style line markers: # <number> "filename"
            // These are emitted by the preprocessor and must not be lexed as tokens.
            // A line marker is a '#' at the start of a line (after optional whitespace)
            // followed by a digit.
            if self.input[self.pos] == b'#' && self.is_line_marker() {
                // Skip the entire line
                while self.pos < self.input.len() && self.input[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }

            // Skip line comments
            if self.pos + 1 < self.input.len() && self.input[self.pos] == b'/' && self.input[self.pos + 1] == b'/' {
                while self.pos < self.input.len() && self.input[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }

            // Skip block comments
            if self.pos + 1 < self.input.len() && self.input[self.pos] == b'/' && self.input[self.pos + 1] == b'*' {
                self.pos += 2;
                while self.pos + 1 < self.input.len() {
                    if self.input[self.pos] == b'*' && self.input[self.pos + 1] == b'/' {
                        self.pos += 2;
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }

            break;
        }
    }

    /// Check if the current position is at a GCC-style line marker.
    /// A line marker is `# <digit>` at the start of a line (i.e., the '#' is
    /// either at position 0 or preceded by a newline).
    fn is_line_marker(&self) -> bool {
        // Must be at '#'
        if self.pos >= self.input.len() || self.input[self.pos] != b'#' {
            return false;
        }
        // '#' must be at the start of a line
        if self.pos > 0 && self.input[self.pos - 1] != b'\n' {
            return false;
        }
        // Next non-space char must be a digit
        let mut j = self.pos + 1;
        while j < self.input.len() && self.input[j] == b' ' {
            j += 1;
        }
        j < self.input.len() && self.input[j].is_ascii_digit()
    }

    fn peek_next(&self) -> Option<u8> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

    fn lex_number(&mut self, start: usize) -> Token {
        if self.pos + 1 < self.input.len() && self.input[self.pos] == b'0'
            && (self.input[self.pos + 1] == b'x' || self.input[self.pos + 1] == b'X')
        {
            return self.lex_hex_number(start);
        }

        if self.pos + 1 < self.input.len() && self.input[self.pos] == b'0'
            && (self.input[self.pos + 1] == b'b' || self.input[self.pos + 1] == b'B')
        {
            return self.lex_binary_number(start);
        }

        if let Some(tok) = self.lex_octal_number(start) {
            return tok;
        }

        self.lex_decimal_number(start)
    }

    /// Lex a hexadecimal integer or hex float literal (0x/0X prefix).
    fn lex_hex_number(&mut self, start: usize) -> Token {
        self.pos += 2;
        let hex_start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_hexdigit() {
            self.pos += 1;
        }

        // Check for hex float: 0x<digits>.<digits>p<exp> or 0x<digits>p<exp>
        let has_dot = self.pos < self.input.len() && self.input[self.pos] == b'.';
        let after_dot_has_p = if has_dot {
            let mut look = self.pos + 1;
            while look < self.input.len() && self.input[look].is_ascii_hexdigit() {
                look += 1;
            }
            look < self.input.len() && (self.input[look] == b'p' || self.input[look] == b'P')
        } else {
            false
        };
        let has_p = self.pos < self.input.len() && (self.input[self.pos] == b'p' || self.input[self.pos] == b'P');

        if has_dot && after_dot_has_p || has_p {
            return self.lex_hex_float(start, hex_start, has_dot);
        }

        // Regular hex integer
        let hex_str = std::str::from_utf8(&self.input[hex_start..self.pos]).unwrap_or("0");
        let value = u64::from_str_radix(hex_str, 16).unwrap_or(0);
        self.finish_int_literal(value, true, start)
    }

    /// Lex a hex float literal: 0x<int_hex>.<frac_hex>p<+/->exp
    fn lex_hex_float(&mut self, start: usize, hex_start: usize, has_dot: bool) -> Token {
        let int_hex = std::str::from_utf8(&self.input[hex_start..self.pos]).unwrap_or("0");

        let frac_hex = if has_dot {
            self.pos += 1; // skip '.'
            let frac_start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_hexdigit() {
                self.pos += 1;
            }
            std::str::from_utf8(&self.input[frac_start..self.pos]).unwrap_or("")
        } else {
            ""
        };

        // Parse 'p'/'P' exponent (mandatory for hex floats)
        let exp: i64 = if self.pos < self.input.len() && (self.input[self.pos] == b'p' || self.input[self.pos] == b'P') {
            self.pos += 1;
            let exp_neg = if self.pos < self.input.len() && self.input[self.pos] == b'-' {
                self.pos += 1;
                true
            } else {
                if self.pos < self.input.len() && self.input[self.pos] == b'+' {
                    self.pos += 1;
                }
                false
            };
            let exp_start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            let exp_str = std::str::from_utf8(&self.input[exp_start..self.pos]).unwrap_or("0");
            let e: i64 = exp_str.parse().unwrap_or(0);
            if exp_neg { -e } else { e }
        } else {
            0
        };

        // Convert hex float to f64: value = (int_part + frac_part) * 2^exp
        let int_val = u64::from_str_radix(int_hex, 16).unwrap_or(0) as f64;
        let frac_val: f64 = if !frac_hex.is_empty() {
            let frac_int = u64::from_str_radix(frac_hex, 16).unwrap_or(0) as f64;
            frac_int / (16.0_f64).powi(frac_hex.len() as i32)
        } else {
            0.0
        };
        let value = (int_val + frac_val) * (2.0_f64).powi(exp as i32);

        // Check float suffix: f/F = float, l/L = long double
        let float_kind = self.parse_simple_float_suffix();
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        match float_kind {
            1 => Token::new(TokenKind::FloatLiteralF32(value), span),
            2 => {
                let hex_text = std::str::from_utf8(&self.input[start..self.pos]).unwrap_or("0x0p0");
                let f128_bytes = crate::common::long_double::parse_long_double_to_f128_bytes(hex_text);
                Token::new(TokenKind::FloatLiteralLongDouble(value, f128_bytes), span)
            }
            _ => Token::new(TokenKind::FloatLiteral(value), span),
        }
    }

    /// Parse a simple float suffix (f/F → 1, l/L → 2, else 0). No imaginary handling.
    fn parse_simple_float_suffix(&mut self) -> u8 {
        if self.pos < self.input.len() && (self.input[self.pos] == b'f' || self.input[self.pos] == b'F') {
            self.pos += 1;
            1
        } else if self.pos < self.input.len() && (self.input[self.pos] == b'l' || self.input[self.pos] == b'L') {
            self.pos += 1;
            2
        } else {
            0
        }
    }

    /// Lex a binary integer literal (0b/0B prefix).
    fn lex_binary_number(&mut self, start: usize) -> Token {
        self.pos += 2;
        let bin_start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos] == b'0' || self.input[self.pos] == b'1') {
            self.pos += 1;
        }
        let bin_str = std::str::from_utf8(&self.input[bin_start..self.pos]).unwrap_or("0");
        let value = u64::from_str_radix(bin_str, 2).unwrap_or(0);
        self.finish_int_literal(value, true, start)
    }

    /// Try to lex an octal literal. Returns None if the token turns out to be decimal/float.
    fn lex_octal_number(&mut self, start: usize) -> Option<Token> {
        if self.input[self.pos] != b'0' || !self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            return None;
        }
        let saved_pos = self.pos;
        self.pos += 1;
        let oct_start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] >= b'0' && self.input[self.pos] <= b'7' {
            self.pos += 1;
        }
        // Float indicator or non-octal digit → backtrack to decimal.
        // But '.' followed by '..' is ellipsis, not a decimal point — keep the octal.
        if self.pos < self.input.len() && matches!(self.input[self.pos], b'.' | b'e' | b'E' | b'8' | b'9') {
            let is_ellipsis = self.input[self.pos] == b'.'
                && self.pos + 2 < self.input.len()
                && self.input[self.pos + 1] == b'.'
                && self.input[self.pos + 2] == b'.';
            if !is_ellipsis {
                self.pos = saved_pos;
                return None;
            }
        }
        let oct_str = std::str::from_utf8(&self.input[oct_start..self.pos]).unwrap_or("0");
        let value = u64::from_str_radix(oct_str, 8).unwrap_or(0);
        Some(self.finish_int_literal(value, true, start))
    }

    /// Common integer literal finish: parse suffix, return token.
    fn finish_int_literal(&mut self, value: u64, is_hex_or_octal: bool, start: usize) -> Token {
        let (is_unsigned, is_long, is_long_long, is_imaginary) = self.parse_int_suffix();
        if is_imaginary {
            let span = Span::new(start as u32, self.pos as u32, self.file_id);
            return Token::new(TokenKind::ImaginaryLiteral(value as f64), span);
        }
        self.make_int_token(value, is_unsigned, is_long, is_long_long, is_hex_or_octal, start)
    }

    /// Lex a decimal integer or float literal.
    fn lex_decimal_number(&mut self, start: usize) -> Token {
        let mut is_float = false;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        // Check for decimal point, but NOT if it's the start of '...' (ellipsis).
        // E.g. `2...15` from GCC case range `case 2 ... 15:` must lex as `2` `...` `15`,
        // not as float `2.` followed by invalid `..15`.
        if self.pos < self.input.len() && self.input[self.pos] == b'.'
            && !(self.pos + 2 < self.input.len() && self.input[self.pos + 1] == b'.' && self.input[self.pos + 2] == b'.')
        {
            is_float = true;
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        if self.pos < self.input.len() && (self.input[self.pos] == b'e' || self.input[self.pos] == b'E') {
            is_float = true;
            self.pos += 1;
            if self.pos < self.input.len() && (self.input[self.pos] == b'+' || self.input[self.pos] == b'-') {
                self.pos += 1;
            }
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        // Save the end position of the number digits before parsing suffixes
        // (which advance self.pos further).
        let num_end = self.pos;

        if is_float {
            let (float_kind, is_imaginary) = self.parse_float_suffix();
            // Borrow the digit text as &str without allocating a String.
            let text = std::str::from_utf8(&self.input[start..num_end]).unwrap_or("0");
            self.make_float_token(text, float_kind, is_imaginary, start)
        } else {
            // Parse the integer value directly from the &str borrow (no heap allocation).
            let text = std::str::from_utf8(&self.input[start..num_end]).unwrap_or("0");
            let uvalue: u64 = text.parse().unwrap_or(0);
            self.finish_int_literal(uvalue, false, start)
        }
    }

    /// Parse float suffix with imaginary support (GCC extension).
    /// Returns (float_kind, is_imaginary) where float_kind: 0=double, 1=float, 2=long double.
    fn parse_float_suffix(&mut self) -> (u8, bool) {
        let mut is_imaginary = false;
        let float_kind = if self.pos < self.input.len() && (self.input[self.pos] == b'f' || self.input[self.pos] == b'F') {
            self.pos += 1;
            if self.pos < self.input.len() && (self.input[self.pos] == b'i' || self.input[self.pos] == b'I') {
                self.pos += 1;
                is_imaginary = true;
            }
            1
        } else if self.pos < self.input.len() && (self.input[self.pos] == b'l' || self.input[self.pos] == b'L') {
            self.pos += 1;
            if self.pos < self.input.len() && (self.input[self.pos] == b'i' || self.input[self.pos] == b'I') {
                self.pos += 1;
                is_imaginary = true;
            }
            2
        } else if self.pos < self.input.len() && (self.input[self.pos] == b'i' || self.input[self.pos] == b'I') {
            self.pos += 1;
            is_imaginary = true;
            if self.pos < self.input.len() && (self.input[self.pos] == b'f' || self.input[self.pos] == b'F') {
                self.pos += 1;
                1
            } else if self.pos < self.input.len() && (self.input[self.pos] == b'l' || self.input[self.pos] == b'L') {
                self.pos += 1;
                2
            } else {
                0
            }
        } else {
            0
        };
        // Also consume trailing 'j'/'J' suffix (C99/GCC alternative for imaginary)
        if !is_imaginary && self.pos < self.input.len() && (self.input[self.pos] == b'j' || self.input[self.pos] == b'J') {
            self.pos += 1;
            is_imaginary = true;
        }
        (float_kind, is_imaginary)
    }

    /// Construct a float/imaginary token from parsed components.
    fn make_float_token(&self, text: &str, float_kind: u8, is_imaginary: bool, start: usize) -> Token {
        let value: f64 = text.parse().unwrap_or(0.0);
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        if is_imaginary {
            match float_kind {
                1 => Token::new(TokenKind::ImaginaryLiteralF32(value), span),
                2 => {
                    let f128_bytes = crate::common::long_double::parse_long_double_to_f128_bytes(text);
                    Token::new(TokenKind::ImaginaryLiteralLongDouble(value, f128_bytes), span)
                }
                _ => Token::new(TokenKind::ImaginaryLiteral(value), span),
            }
        } else {
            match float_kind {
                1 => Token::new(TokenKind::FloatLiteralF32(value), span),
                2 => {
                    let f128_bytes = crate::common::long_double::parse_long_double_to_f128_bytes(text);
                    Token::new(TokenKind::FloatLiteralLongDouble(value, f128_bytes), span)
                }
                _ => Token::new(TokenKind::FloatLiteral(value), span),
            }
        }
    }

    /// Parse integer suffix and return (is_unsigned, is_long, is_imaginary).
    /// is_long is true for l/L or ll/LL suffixes.
    /// is_imaginary is true for trailing 'i' suffix (GCC extension: 5i).
    /// Parse integer suffix and return (is_unsigned, is_long, is_long_long, is_imaginary).
    /// is_long is true for single l/L suffix. is_long_long is true for ll/LL suffix.
    fn parse_int_suffix(&mut self) -> (bool, bool, bool, bool) {
        let mut is_imaginary = false;
        // First check for standalone 'i'/'I' imaginary suffix (GCC extension: 5i, 5I)
        // Must check this before the main loop since 'i'/'I' alone means imaginary, not a regular suffix
        if self.pos < self.input.len() && (self.input[self.pos] == b'i' || self.input[self.pos] == b'I') {
            // Check it's not the start of an identifier (like 'int')
            let next = if self.pos + 1 < self.input.len() { self.input[self.pos + 1] } else { 0 };
            if !next.is_ascii_alphanumeric() && next != b'_' {
                self.pos += 1; // consume 'i'/'I' as imaginary suffix
                return (false, false, false, true);
            }
        }

        let mut is_unsigned = false;
        let mut is_long = false;
        let mut is_long_long = false;
        loop {
            if self.pos < self.input.len() && (self.input[self.pos] == b'u' || self.input[self.pos] == b'U') {
                is_unsigned = true;
                self.pos += 1;
            } else if self.pos < self.input.len() && (self.input[self.pos] == b'l' || self.input[self.pos] == b'L') {
                self.pos += 1;
                // Check for second l/L for ll/LL
                if self.pos < self.input.len() && (self.input[self.pos] == b'l' || self.input[self.pos] == b'L') {
                    is_long_long = true;
                    self.pos += 1;
                } else {
                    is_long = true;
                }
            } else {
                break;
            }
        }
        // Consume trailing 'i'/'I'/'j'/'J' for GCC imaginary suffix (e.g., 5li, 5ui, 5ULi, 5I)
        if self.pos < self.input.len() && (self.input[self.pos] == b'i' || self.input[self.pos] == b'I' || self.input[self.pos] == b'j' || self.input[self.pos] == b'J') {
            let next = if self.pos + 1 < self.input.len() { self.input[self.pos + 1] } else { 0 };
            if !next.is_ascii_alphanumeric() && next != b'_' {
                self.pos += 1;
                is_imaginary = true;
            }
        }
        (is_unsigned, is_long, is_long_long, is_imaginary)
    }

    /// Create the appropriate token kind based on integer value, suffix, and base info.
    /// For hex/octal literals, C promotes: int -> unsigned int -> long -> unsigned long -> long long -> unsigned long long.
    /// For decimal literals: int -> long -> long long (no implicit unsigned).
    fn make_int_token(&self, value: u64, is_unsigned: bool, is_long: bool, is_long_long: bool, is_hex_or_octal: bool, start: usize) -> Token {
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        if is_unsigned && is_long_long {
            // Explicit ULL suffix: always unsigned long long (64-bit)
            Token::new(TokenKind::ULongLongLiteral(value), span)
        } else if is_unsigned && is_long {
            // Explicit UL suffix: unsigned long
            Token::new(TokenKind::ULongLiteral(value), span)
        } else if is_unsigned {
            if value > u32::MAX as u64 {
                Token::new(TokenKind::ULongLiteral(value), span)
            } else {
                Token::new(TokenKind::UIntLiteral(value), span)
            }
        } else if is_long_long {
            // Explicit LL suffix: always long long (64-bit)
            if is_hex_or_octal && value > i64::MAX as u64 {
                Token::new(TokenKind::ULongLongLiteral(value), span)
            } else {
                Token::new(TokenKind::LongLongLiteral(value as i64), span)
            }
        } else if is_long {
            // Explicit L suffix: C11 6.4.4.1 type promotion for hex/octal:
            //   long -> unsigned long -> long long -> unsigned long long
            // On ILP32, long is 32-bit so values > i32::MAX need promotion.
            if is_hex_or_octal && crate::common::types::target_is_32bit() {
                if value <= i32::MAX as u64 {
                    Token::new(TokenKind::LongLiteral(value as i64), span)
                } else if value <= u32::MAX as u64 {
                    Token::new(TokenKind::ULongLiteral(value), span)
                } else if value <= i64::MAX as u64 {
                    Token::new(TokenKind::LongLongLiteral(value as i64), span)
                } else {
                    Token::new(TokenKind::ULongLongLiteral(value), span)
                }
            } else if is_hex_or_octal && value > i64::MAX as u64 {
                // LP64: long is 64-bit, value exceeds signed range -> unsigned long
                Token::new(TokenKind::ULongLiteral(value), span)
            } else {
                Token::new(TokenKind::LongLiteral(value as i64), span)
            }
        } else if is_hex_or_octal {
            // Hex/octal: int -> unsigned int -> long -> unsigned long
            if value <= i32::MAX as u64 {
                Token::new(TokenKind::IntLiteral(value as i64), span)
            } else if value <= u32::MAX as u64 {
                Token::new(TokenKind::UIntLiteral(value), span)
            } else if value <= i64::MAX as u64 {
                Token::new(TokenKind::LongLiteral(value as i64), span)
            } else {
                Token::new(TokenKind::ULongLiteral(value), span)
            }
        } else {
            // Decimal with no suffix: C11 6.4.4.1 Table 6
            // Type sequence: int -> long int -> long long int
            if value > i64::MAX as u64 {
                // Doesn't fit in any signed type; implementation-defined, use unsigned long
                Token::new(TokenKind::ULongLiteral(value), span)
            } else if crate::common::types::target_is_32bit() {
                // ILP32: int (32) -> long (32) -> long long (64)
                if value <= i32::MAX as u64 {
                    Token::new(TokenKind::IntLiteral(value as i64), span)
                } else {
                    // Doesn't fit in int or long (both 32-bit), promote to long long
                    Token::new(TokenKind::LongLongLiteral(value as i64), span)
                }
            } else {
                // LP64: int (32) -> long (64)
                if value <= i32::MAX as u64 {
                    Token::new(TokenKind::IntLiteral(value as i64), span)
                } else {
                    // Doesn't fit in int, promote to long
                    Token::new(TokenKind::LongLiteral(value as i64), span)
                }
            }
        }
    }

    fn lex_string(&mut self, start: usize) -> Token {
        self.pos += 1; // skip opening "
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != b'"' {
            if self.input[self.pos] == b'\\' {
                self.pos += 1;
                if self.pos < self.input.len() {
                    let ch = self.lex_escape_char();
                    // C narrow strings store raw bytes, so Unicode escapes (\u, \U)
                    // must be UTF-8 encoded to match GCC/Clang behavior.
                    if (ch as u32) > 0xFF {
                        let mut buf = [0u8; 4];
                        let utf8 = ch.encode_utf8(&mut buf);
                        for byte in utf8.bytes() {
                            s.push(byte as char);
                        }
                    } else {
                        s.push(ch);
                    }
                }
            } else {
                // Decode PUA-encoded bytes back to original values for
                // non-UTF-8 source files (e.g., EUC-JP string literals)
                let (byte, consumed) = decode_pua_byte(&self.input, self.pos);
                s.push(byte as char);
                self.pos += consumed;
            }
        }
        if self.pos < self.input.len() {
            self.pos += 1; // skip closing "
        }
        Token::new(TokenKind::StringLiteral(s), Span::new(start as u32, self.pos as u32, self.file_id))
    }

    fn lex_wide_string(&mut self, start: usize) -> Token {
        self.pos += 1; // skip opening "
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != b'"' {
            if self.input[self.pos] == b'\\' {
                self.pos += 1;
                if self.pos < self.input.len() {
                    // Wide strings store code points directly (no UTF-8 encoding needed)
                    s.push(self.lex_escape_char());
                }
            } else {
                // Check for PUA-encoded bytes first (from non-UTF-8 source files)
                let (byte, consumed) = decode_pua_byte(&self.input, self.pos);
                if consumed > 1 {
                    // PUA byte: decode back to original byte value
                    s.push(byte as char);
                    self.pos += consumed;
                } else {
                    // Decode UTF-8 character for wide string
                    let byte = self.input[self.pos];
                    if byte < 0x80 {
                        s.push(byte as char);
                        self.pos += 1;
                    } else {
                        // Multi-byte UTF-8: decode to a single Unicode code point
                        let remaining = &self.input[self.pos..];
                        let end = std::cmp::min(4, remaining.len());
                        if let Ok(text) = std::str::from_utf8(&remaining[..end]) {
                            if let Some(ch) = text.chars().next() {
                                s.push(ch);
                                self.pos += ch.len_utf8();
                            } else {
                                s.push(byte as char);
                                self.pos += 1;
                            }
                        } else if let Ok(text) = std::str::from_utf8(remaining) {
                            if let Some(ch) = text.chars().next() {
                                s.push(ch);
                                self.pos += ch.len_utf8();
                            } else {
                                s.push(byte as char);
                                self.pos += 1;
                            }
                        } else {
                            s.push(byte as char);
                            self.pos += 1;
                        }
                    }
                }
            }
        }
        if self.pos < self.input.len() {
            self.pos += 1; // skip closing "
        }
        Token::new(TokenKind::WideStringLiteral(s), Span::new(start as u32, self.pos as u32, self.file_id))
    }

    /// Lex a u"..." char16_t string literal. Same parsing as wide string but produces
    /// Char16StringLiteral token. The Rust String stores Unicode chars; the downstream
    /// pipeline converts each to a u16 value (truncating code points > 0xFFFF).
    fn lex_char16_string(&mut self, start: usize) -> Token {
        self.pos += 1; // skip opening "
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != b'"' {
            if self.input[self.pos] == b'\\' {
                self.pos += 1;
                if self.pos < self.input.len() {
                    s.push(self.lex_escape_char());
                }
            } else {
                // Check for PUA-encoded bytes first
                let (byte, consumed) = decode_pua_byte(&self.input, self.pos);
                if consumed > 1 {
                    s.push(byte as char);
                    self.pos += consumed;
                } else {
                    let byte = self.input[self.pos];
                    if byte < 0x80 {
                        s.push(byte as char);
                        self.pos += 1;
                    } else {
                        let remaining = &self.input[self.pos..];
                        let end = std::cmp::min(4, remaining.len());
                        if let Ok(text) = std::str::from_utf8(&remaining[..end]) {
                            if let Some(ch) = text.chars().next() {
                                s.push(ch);
                                self.pos += ch.len_utf8();
                            } else {
                                s.push(byte as char);
                                self.pos += 1;
                            }
                        } else if let Ok(text) = std::str::from_utf8(remaining) {
                            if let Some(ch) = text.chars().next() {
                                s.push(ch);
                                self.pos += ch.len_utf8();
                            } else {
                                s.push(byte as char);
                                self.pos += 1;
                            }
                        } else {
                            s.push(byte as char);
                            self.pos += 1;
                        }
                    }
                }
            }
        }
        if self.pos < self.input.len() {
            self.pos += 1; // skip closing "
        }
        Token::new(TokenKind::Char16StringLiteral(s), Span::new(start as u32, self.pos as u32, self.file_id))
    }

    fn lex_wide_char(&mut self, start: usize) -> Token {
        self.pos += 1; // skip opening '
        let mut value: u32 = 0;
        if self.pos < self.input.len() && self.input[self.pos] != b'\'' {
            if self.input[self.pos] == b'\\' {
                self.pos += 1;
                let ch = self.lex_escape_char();
                value = ch as u32; // Unicode escapes return code point directly
            } else {
                // Check for PUA-encoded bytes first
                let (byte, consumed) = decode_pua_byte(&self.input, self.pos);
                if consumed > 1 {
                    value = byte as u32;
                    self.pos += consumed;
                } else {
                    // Decode UTF-8 to get Unicode code point
                    let byte = self.input[self.pos];
                    if byte < 0x80 {
                        value = byte as u32;
                        self.pos += 1;
                    } else {
                        let remaining = &self.input[self.pos..];
                        let end = std::cmp::min(remaining.len(), 4);
                        if let Ok(text) = std::str::from_utf8(&remaining[..end]) {
                            if let Some(ch) = text.chars().next() {
                                value = ch as u32;
                                self.pos += ch.len_utf8();
                            }
                        } else if let Ok(text) = std::str::from_utf8(remaining) {
                            if let Some(ch) = text.chars().next() {
                                value = ch as u32;
                                self.pos += ch.len_utf8();
                            }
                        } else {
                            value = byte as u32;
                            self.pos += 1;
                        }
                    }
                }
            }
        }
        // Skip any remaining chars until closing quote
        while self.pos < self.input.len() && self.input[self.pos] != b'\'' {
            self.pos += 1;
        }
        if self.pos < self.input.len() && self.input[self.pos] == b'\'' {
            self.pos += 1; // skip closing '
        }
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        // Wide char literals have type int (wchar_t)
        Token::new(TokenKind::IntLiteral(value as i64), span)
    }

    fn lex_char(&mut self, start: usize) -> Token {
        self.pos += 1; // skip opening '
        let mut value: i32 = 0;
        let mut char_count = 0;
        while self.pos < self.input.len() && self.input[self.pos] != b'\'' {
            let ch = if self.input[self.pos] == b'\\' {
                self.pos += 1;
                self.lex_escape_char()
            } else {
                // Decode PUA-encoded bytes for non-UTF-8 source files
                let (byte, consumed) = decode_pua_byte(&self.input, self.pos);
                self.pos += consumed;
                byte as char
            };
            // C narrow char literals encode Unicode escapes as UTF-8 bytes
            // combined into a multi-byte int value, matching GCC behavior.
            if (ch as u32) > 0xFF {
                let mut buf = [0u8; 4];
                let utf8 = ch.encode_utf8(&mut buf);
                for byte in utf8.bytes() {
                    value = (value << 8) | (byte as i32);
                    char_count += 1;
                }
            } else {
                // Multi-character constant: shift previous value and add new byte
                value = (value << 8) | (ch as u8 as i32);
                char_count += 1;
            }
        }
        if self.pos < self.input.len() && self.input[self.pos] == b'\'' {
            self.pos += 1; // skip closing '
        }
        let span = Span::new(start as u32, self.pos as u32, self.file_id);
        if char_count <= 1 {
            // Single character: use CharLiteral with the char value
            let ch = if value == 0 { '\0' } else { (value as u8) as char };
            Token::new(TokenKind::CharLiteral(ch), span)
        } else {
            // Multi-character constant: produce an IntLiteral with the combined value
            Token::new(TokenKind::IntLiteral(value as i64), span)
        }
    }

    fn lex_escape_char(&mut self) -> char {
        if self.pos >= self.input.len() {
            return '\0';
        }
        let ch = self.input[self.pos];
        self.pos += 1;
        match ch {
            b'n' => '\n',
            b't' => '\t',
            b'r' => '\r',
            b'\\' => '\\',
            b'\'' => '\'',
            b'"' => '"',
            b'a' => '\x07',
            b'b' => '\x08',
            b'e' | b'E' => '\x1b', // GNU extension: ESC
            b'f' => '\x0c',
            b'v' => '\x0b',
            b'x' => {
                // Hex escape: \xNN - consumes all hex digits, value truncated to byte
                let mut val = 0u32;
                while self.pos < self.input.len() && self.input[self.pos].is_ascii_hexdigit() {
                    val = val * 16 + hex_digit_val(self.input[self.pos]) as u32;
                    self.pos += 1;
                }
                // Truncate to byte value and use direct char mapping to avoid
                // multi-byte UTF-8 encoding issues with values > 127
                (val as u8) as char
            }
            b'u' => {
                // Universal character name: \uNNNN (exactly 4 hex digits)
                self.lex_unicode_escape(4)
            }
            b'U' => {
                // Universal character name: \UNNNNNNNN (exactly 8 hex digits)
                self.lex_unicode_escape(8)
            }
            b'0'..=b'7' => {
                // Octal escape: \0 through \377 (1-3 octal digits)
                // Note: \0 alone produces null; \040 produces space (32), etc.
                let mut val = (ch - b'0') as u32;
                for _ in 0..2 {
                    if self.pos < self.input.len() && self.input[self.pos] >= b'0' && self.input[self.pos] <= b'7' {
                        val = val * 8 + (self.input[self.pos] - b'0') as u32;
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                // Truncate to byte value to match C semantics
                (val as u8) as char
            }
            _ => ch as char,
        }
    }

    /// Parse a universal character name (\uNNNN or \UNNNNNNNN).
    /// `num_digits` is 4 for \u or 8 for \U.
    /// Returns the Unicode code point as a Rust char.
    // TODO: C11 requires exactly num_digits hex digits; emit diagnostic if fewer provided.
    // TODO: C11 §6.4.3 disallows certain code points (below 0x00A0 except 0x24/0x40/0x60,
    //       and surrogates 0xD800-0xDFFF). Validate and emit diagnostics for these.
    fn lex_unicode_escape(&mut self, num_digits: usize) -> char {
        let mut val = 0u32;
        for _ in 0..num_digits {
            if self.pos < self.input.len() && self.input[self.pos].is_ascii_hexdigit() {
                val = val * 16 + hex_digit_val(self.input[self.pos]) as u32;
                self.pos += 1;
            } else {
                break;
            }
        }
        // TODO: Emit a diagnostic for invalid code points (e.g. surrogates) instead of
        //       silently using the replacement character.
        char::from_u32(val).unwrap_or('\u{FFFD}')
    }

    fn lex_identifier(&mut self, start: usize) -> Token {
        while self.pos < self.input.len() && (self.input[self.pos] == b'_' || self.input[self.pos] == b'$' || self.input[self.pos].is_ascii_alphanumeric()) {
            self.pos += 1;
        }

        // Check for wide/unicode char/string prefixes: L'x', L"...", u'x', u"...", U'x', U"..."
        if self.pos < self.input.len() {
            let text_len = self.pos - start;
            let next = self.input[self.pos];
            if next == b'\'' || next == b'"' {
                let prefix = &self.input[start..self.pos];
                let is_wide_prefix = match text_len {
                    1 => prefix[0] == b'L' || prefix[0] == b'u' || prefix[0] == b'U',
                    2 => prefix == b"u8",
                    _ => false,
                };
                if is_wide_prefix {
                    if next == b'\'' {
                        // Wide/unicode char literal: value is the Unicode code point
                        return self.lex_wide_char(start);
                    } else {
                        // Wide/unicode string literal: L"...", u"...", U"...", u8"..."
                        let is_wide_32 = text_len == 1 && (prefix[0] == b'L' || prefix[0] == b'U');
                        let is_char16 = text_len == 1 && prefix[0] == b'u';
                        if is_wide_32 {
                            return self.lex_wide_string(start);
                        } else if is_char16 {
                            // u"..." - char16_t string (16-bit elements)
                            return self.lex_char16_string(start);
                        } else {
                            // u8"..." - UTF-8 string, same as narrow string
                            return self.lex_string(start);
                        }
                    }
                }
            }
        }

        let text = std::str::from_utf8(&self.input[start..self.pos]).unwrap_or("");
        let span = Span::new(start as u32, self.pos as u32, self.file_id);

        // Check for synthetic pragma pack directives emitted by preprocessor
        if let Some(pack_tok) = Self::try_pragma_pack_token(text) {
            return Token::new(pack_tok, span);
        }

        // Check for synthetic pragma visibility directives emitted by preprocessor
        if let Some(vis_tok) = Self::try_pragma_visibility_token(text) {
            return Token::new(vis_tok, span);
        }

        if let Some(kw) = TokenKind::from_keyword(text, self.gnu_extensions) {
            Token::new(kw, span)
        } else {
            Token::new(TokenKind::Identifier(text.to_string()), span)
        }
    }

    /// Recognize synthetic pragma pack identifiers emitted by the preprocessor.
    /// Format: __ccc_pack_set_N, __ccc_pack_push_N, __ccc_pack_push_only,
    ///         __ccc_pack_pop, __ccc_pack_reset
    fn try_pragma_pack_token(text: &str) -> Option<TokenKind> {
        if let Some(rest) = text.strip_prefix("__ccc_pack_") {
            if rest == "pop" {
                Some(TokenKind::PragmaPackPop)
            } else if rest == "reset" {
                Some(TokenKind::PragmaPackReset)
            } else if rest == "push_only" {
                Some(TokenKind::PragmaPackPushOnly)
            } else if let Some(n_str) = rest.strip_prefix("set_") {
                if let Ok(n) = n_str.parse::<usize>() {
                    Some(TokenKind::PragmaPackSet(n))
                } else {
                    None
                }
            } else if let Some(n_str) = rest.strip_prefix("push_") {
                if let Ok(n) = n_str.parse::<usize>() {
                    Some(TokenKind::PragmaPackPush(n))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Recognize synthetic pragma visibility identifiers emitted by the preprocessor.
    /// Format: __ccc_visibility_push_VISIBILITY, __ccc_visibility_pop
    fn try_pragma_visibility_token(text: &str) -> Option<TokenKind> {
        if let Some(rest) = text.strip_prefix("__ccc_visibility_") {
            if rest == "pop" {
                Some(TokenKind::PragmaVisibilityPop)
            } else if let Some(vis) = rest.strip_prefix("push_") {
                match vis {
                    "hidden" | "default" | "protected" | "internal" => {
                        Some(TokenKind::PragmaVisibilityPush(vis.to_string()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn lex_punctuation(&mut self, start: usize) -> Token {
        let ch = self.input[self.pos];
        self.pos += 1;

        let kind = match ch {
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b';' => TokenKind::Semicolon,
            b',' => TokenKind::Comma,
            b'~' => TokenKind::Tilde,
            b'?' => TokenKind::Question,
            b':' => TokenKind::Colon,
            b'#' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'#' {
                    self.pos += 1;
                    TokenKind::HashHash
                } else {
                    TokenKind::Hash
                }
            }
            b'.' => {
                if self.pos + 1 < self.input.len() && self.input[self.pos] == b'.' && self.input[self.pos + 1] == b'.' {
                    self.pos += 2;
                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            b'+' => {
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        b'+' => { self.pos += 1; TokenKind::PlusPlus }
                        b'=' => { self.pos += 1; TokenKind::PlusAssign }
                        _ => TokenKind::Plus,
                    }
                } else {
                    TokenKind::Plus
                }
            }
            b'-' => {
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        b'-' => { self.pos += 1; TokenKind::MinusMinus }
                        b'=' => { self.pos += 1; TokenKind::MinusAssign }
                        b'>' => { self.pos += 1; TokenKind::Arrow }
                        _ => TokenKind::Minus,
                    }
                } else {
                    TokenKind::Minus
                }
            }
            b'*' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                    self.pos += 1;
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            b'/' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                    self.pos += 1;
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            b'%' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                    self.pos += 1;
                    TokenKind::PercentAssign
                } else {
                    TokenKind::Percent
                }
            }
            b'&' => {
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        b'&' => { self.pos += 1; TokenKind::AmpAmp }
                        b'=' => { self.pos += 1; TokenKind::AmpAssign }
                        _ => TokenKind::Amp,
                    }
                } else {
                    TokenKind::Amp
                }
            }
            b'|' => {
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        b'|' => { self.pos += 1; TokenKind::PipePipe }
                        b'=' => { self.pos += 1; TokenKind::PipeAssign }
                        _ => TokenKind::Pipe,
                    }
                } else {
                    TokenKind::Pipe
                }
            }
            b'^' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                    self.pos += 1;
                    TokenKind::CaretAssign
                } else {
                    TokenKind::Caret
                }
            }
            b'!' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                    self.pos += 1;
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            b'=' => {
                if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                    self.pos += 1;
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Assign
                }
            }
            b'<' => {
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        b'<' => {
                            self.pos += 1;
                            if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                                self.pos += 1;
                                TokenKind::LessLessAssign
                            } else {
                                TokenKind::LessLess
                            }
                        }
                        b'=' => { self.pos += 1; TokenKind::LessEqual }
                        _ => TokenKind::Less,
                    }
                } else {
                    TokenKind::Less
                }
            }
            b'>' => {
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        b'>' => {
                            self.pos += 1;
                            if self.pos < self.input.len() && self.input[self.pos] == b'=' {
                                self.pos += 1;
                                TokenKind::GreaterGreaterAssign
                            } else {
                                TokenKind::GreaterGreater
                            }
                        }
                        b'=' => { self.pos += 1; TokenKind::GreaterEqual }
                        _ => TokenKind::Greater,
                    }
                } else {
                    TokenKind::Greater
                }
            }
            _ => {
                // Non-ASCII or unknown character: skip any remaining bytes of
                // a multi-byte UTF-8 sequence (including PUA-encoded bytes from
                // non-UTF-8 source files) and continue tokenizing.
                // TODO: emit a diagnostic for genuinely unknown/invalid characters
                while self.pos < self.input.len() && (self.input[self.pos] & 0xC0) == 0x80 {
                    self.pos += 1; // skip continuation bytes
                }
                return self.next_token();
            }
        };

        Token::new(kind, Span::new(start as u32, self.pos as u32, self.file_id))
    }
}

fn hex_digit_val(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PBT properties (round 02) — see pbt-out/rounds/02_lexer/PROPERTIES.md
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod pbt_tests {
    use super::Lexer;
    use crate::frontend::lexer::token::TokenKind;
    use proptest::prelude::*;

    fn kinds_of(input: &str) -> Vec<TokenKind> {
        Lexer::new(input, 0).tokenize().into_iter().map(|t| t.kind).collect()
    }

    fn first_tok(input: &str) -> TokenKind {
        let ks = kinds_of(input);
        assert!(!ks.is_empty(), "no tokens for {input:?}");
        ks[0].clone()
    }

    fn int_payload(k: &TokenKind) -> u64 {
        match k {
            TokenKind::IntLiteral(v)
            | TokenKind::LongLiteral(v)
            | TokenKind::LongLongLiteral(v) => *v as u64,
            TokenKind::UIntLiteral(v)
            | TokenKind::ULongLiteral(v)
            | TokenKind::ULongLongLiteral(v) => *v,
            other => panic!("not an int token: {other:?}"),
        }
    }

    fn float_payload(k: &TokenKind) -> f64 {
        match k {
            TokenKind::FloatLiteral(v)
            | TokenKind::FloatLiteralF32(v)
            | TokenKind::FloatLiteralLongDouble(v, _) => *v,
            other => panic!("not a float token: {other:?}"),
        }
    }

    /// Reference C11 §6.4.4.1 promotion under LP64, written from README:218-226
    /// (int=32, long=64, long long=64; hex/octal/bin try unsigned intermediates).
    fn expect_int_kind(v: u64, suffix: &str, base: &str) -> TokenKind {
        let hexish = matches!(base, "hex" | "oct" | "bin");
        match (suffix, hexish) {
            ("ull", _) => TokenKind::ULongLongLiteral(v),
            ("ul", _) => TokenKind::ULongLiteral(v),
            ("u", _) => {
                if v <= u32::MAX as u64 {
                    TokenKind::UIntLiteral(v)
                } else {
                    TokenKind::ULongLiteral(v)
                }
            }
            ("ll", true) => {
                if v > i64::MAX as u64 {
                    TokenKind::ULongLongLiteral(v)
                } else {
                    TokenKind::LongLongLiteral(v as i64)
                }
            }
            ("ll", false) => TokenKind::LongLongLiteral(v as i64),
            ("l", true) => {
                if v > i64::MAX as u64 {
                    TokenKind::ULongLiteral(v)
                } else {
                    TokenKind::LongLiteral(v as i64)
                }
            }
            ("l", false) => TokenKind::LongLiteral(v as i64),
            (_, true) => {
                if v <= i32::MAX as u64 {
                    TokenKind::IntLiteral(v as i64)
                } else if v <= u32::MAX as u64 {
                    TokenKind::UIntLiteral(v)
                } else if v <= i64::MAX as u64 {
                    TokenKind::LongLiteral(v as i64)
                } else {
                    TokenKind::ULongLiteral(v)
                }
            }
            (_, false) => {
                if v > i64::MAX as u64 {
                    TokenKind::ULongLiteral(v)
                } else if v <= i32::MAX as u64 {
                    TokenKind::IntLiteral(v as i64)
                } else {
                    TokenKind::LongLiteral(v as i64)
                }
            }
        }
    }

    fn render_int(v: u64, suffix: &str, base: &str) -> String {
        match base {
            "hex" => format!("0x{:x}{}", v, suffix),
            "oct" => format!("0{:o}{}", v, suffix),
            "bin" => format!("0b{:b}{}", v, suffix),
            _ => format!("{}{}", v, suffix),
        }
    }

    // P1 ─ identifier round-trip ────────────────────────────────────────────
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p1_identifier_round_trip(
            first in 0usize..54,
            rest in prop::collection::vec(0usize..64, 0..15),
        ) {
            const HEAD: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_$";
            const TAIL: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_$0123456789";
            let mut s = String::new();
            s.push(HEAD[first % HEAD.len()] as char);
            for &i in &rest {
                s.push(TAIL[i % TAIL.len()] as char);
            }
            prop_assume!(TokenKind::from_keyword(&s, true).is_none());
            prop_assume!(TokenKind::from_keyword(&s, false).is_none());
            prop_assume!(!s.starts_with("__ccc_pack_") && !s.starts_with("__ccc_visibility_"));
            let toks = Lexer::new(&s, 7).tokenize();
            prop_assert_eq!(toks.len(), 2);
            prop_assert_eq!(&toks[0].kind, &TokenKind::Identifier(s.clone()));
            prop_assert_eq!(toks[0].span.start, 0);
            prop_assert_eq!(toks[0].span.end as usize, s.len());
            prop_assert_eq!(toks[0].span.file_id, 7);
            prop_assert!(toks[1].is_eof());
            prop_assert_eq!(toks[1].span.start as usize, s.len());
            prop_assert_eq!(toks[1].span.end as usize, s.len());
        }
    }

    // P2 ─ keyword table vs C11 §6.4.1 + documented gnu_extensions contract ──
    #[test]
    fn p2_keyword_table_reference() {
        // (canonical spelling, expected kind, accepted in strict mode)
        let table: &[(&str, TokenKind, bool)] = &[
            ("auto", TokenKind::Auto, true), ("break", TokenKind::Break, true),
            ("case", TokenKind::Case, true), ("char", TokenKind::Char, true),
            ("const", TokenKind::Const, true), ("continue", TokenKind::Continue, true),
            ("default", TokenKind::Default, true), ("do", TokenKind::Do, true),
            ("double", TokenKind::Double, true), ("else", TokenKind::Else, true),
            ("enum", TokenKind::Enum, true), ("extern", TokenKind::Extern, true),
            ("float", TokenKind::Float, true), ("for", TokenKind::For, true),
            ("goto", TokenKind::Goto, true), ("if", TokenKind::If, true),
            ("inline", TokenKind::Inline, true), ("int", TokenKind::Int, true),
            ("long", TokenKind::Long, true), ("register", TokenKind::Register, true),
            ("restrict", TokenKind::Restrict, true), ("return", TokenKind::Return, true),
            ("short", TokenKind::Short, true), ("signed", TokenKind::Signed, true),
            ("sizeof", TokenKind::Sizeof, true), ("static", TokenKind::Static, true),
            ("struct", TokenKind::Struct, true), ("switch", TokenKind::Switch, true),
            ("typedef", TokenKind::Typedef, true), ("union", TokenKind::Union, true),
            ("unsigned", TokenKind::Unsigned, true), ("void", TokenKind::Void, true),
            ("volatile", TokenKind::Volatile, true), ("while", TokenKind::While, true),
            ("_Alignas", TokenKind::Alignas, true), ("_Alignof", TokenKind::Alignof, true),
            ("_Atomic", TokenKind::Atomic, true), ("_Bool", TokenKind::Bool, true),
            ("_Complex", TokenKind::Complex, true), ("_Generic", TokenKind::Generic, true),
            ("_Imaginary", TokenKind::Imaginary, true), ("_Noreturn", TokenKind::Noreturn, true),
            ("_Static_assert", TokenKind::StaticAssert, true),
            ("_Thread_local", TokenKind::ThreadLocal, true),
            // GCC canonical spellings (bare GNU words NOT keywords in strict mode)
            ("typeof", TokenKind::Typeof, false), ("asm", TokenKind::Asm, false),
            ("__attribute__", TokenKind::Attribute, true),
            ("__extension__", TokenKind::Extension, true),
            ("__builtin_va_list", TokenKind::Builtin, true),
            ("__builtin_va_arg", TokenKind::BuiltinVaArg, true),
            ("__builtin_types_compatible_p", TokenKind::BuiltinTypesCompatibleP, true),
            ("__int128", TokenKind::Int128, true), ("__uint128_t", TokenKind::UInt128, true),
            ("__real__", TokenKind::RealPart, true), ("__imag__", TokenKind::ImagPart, true),
            ("__auto_type", TokenKind::AutoType, true),
            ("__alignof__", TokenKind::GnuAlignof, true),
            ("__label__", TokenKind::GnuLabel, true),
            ("__seg_gs", TokenKind::SegGs, true), ("__seg_fs", TokenKind::SegFs, true),
            // double-underscore forms are always keywords (token.rs:367 doc)
            ("__typeof__", TokenKind::Typeof, true), ("__asm__", TokenKind::Asm, true),
            ("__volatile__", TokenKind::Volatile, true), ("__const__", TokenKind::Const, true),
            ("__inline__", TokenKind::Inline, true), ("__restrict__", TokenKind::Restrict, true),
            ("__signed__", TokenKind::Signed, true), ("__complex__", TokenKind::Complex, true),
            ("__noreturn__", TokenKind::Noreturn, true), ("__thread", TokenKind::ThreadLocal, true),
        ];
        for (spelling, kind, strict_ok) in table {
            assert_eq!(TokenKind::from_keyword(spelling, true), Some(kind.clone()),
                "gnu-mode keyword mismatch for {spelling:?}");
            assert_eq!(first_tok(spelling), kind.clone(), "tokenize mismatch for {spelling:?}");
            // Display shows the CANONICAL keyword text (token.rs:200-203 doc);
            // double-underscore alias spellings display as their bare form.
            const ALIASES: &[&str] = &[
                "__typeof__", "__asm__", "__volatile__", "__const__", "__inline__",
                "__restrict__", "__signed__", "__complex__", "__noreturn__", "__thread",
            ];
            if !ALIASES.contains(spelling) {
                assert_eq!(format!("{kind}"), format!("'{spelling}'"),
                    "Display mismatch for {spelling:?}");
            }
            let strict = TokenKind::from_keyword(spelling, false);
            if *strict_ok {
                assert_eq!(strict, Some(kind.clone()), "strict-mode should keep {spelling:?}");
            } else {
                assert_eq!(strict, None, "strict-mode must not keep bare GNU word {spelling:?}");
                let mut lx = Lexer::new(spelling, 0);
                lx.set_gnu_extensions(false);
                let ks = lx.tokenize();
                assert_eq!(ks[0].kind, TokenKind::Identifier(spelling.to_string()),
                    "strict-mode tokenize for {spelling:?}");
            }
        }
    }

    // P3 ─ integer literal round-trip with documented promotion (LP64) ──────
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p3_int_round_trip(v in any::<u64>(), suffix in 0usize..6, base in 0usize..4) {
            const SUFS: [&str; 6] = ["", "u", "l", "ll", "ul", "ull"];
            const BASES: [&str; 4] = ["dec", "hex", "oct", "bin"];
            let suffix = SUFS[suffix];
            let base = BASES[base];
            // signed renders must keep v within the documented domain (v <= i64::MAX)
            let v = if matches!(suffix, "l" | "ll") && base == "dec" { v % (1u64 << 63) } else { v };
            let text = render_int(v, suffix, base);
            let toks = Lexer::new(&text, 0).tokenize();
            prop_assert_eq!(toks.len(), 2, "text={:?}", text);
            prop_assert_eq!(&toks[0].kind, &expect_int_kind(v, suffix, base), "text={:?}", text);
            prop_assert_eq!(toks[0].span.end as usize, text.len());
            prop_assert!(toks[1].is_eof());
        }
    }

    // P4 ─ documented promotion boundaries, sampled exactly (exhaustive) ────
    #[test]
    fn p4_promotion_boundaries_exact() {
        let bounds: [u64; 7] = [
            i32::MAX as u64,
            i32::MAX as u64 + 1,
            u32::MAX as u64,
            u32::MAX as u64 + 1,
            i64::MAX as u64,
            i64::MAX as u64 + 1,
            u64::MAX,
        ];
        for &v in &bounds {
            for suffix in ["", "u", "l", "ll", "ul", "ull"] {
                for base in ["dec", "hex", "oct", "bin"] {
                    if matches!(suffix, "l" | "ll") && base == "dec" && v > i64::MAX as u64 {
                        continue; // outside documented decimal-L domain (README silent)
                    }
                    let text = render_int(v, suffix, base);
                    assert_eq!(first_tok(&text), expect_int_kind(v, suffix, base),
                        "boundary v={v} suffix={suffix:?} base={base} text={text:?}");
                }
            }
        }
    }

    // P5 ─ hex float exact value (reference: README:181 formula, exact domain)─
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p5_hex_float_exact(
            int_digits in prop::collection::vec(0usize..16, 1..8),
            frac_digits in prop::collection::vec(0usize..16, 0..4),
            exp in -60i32..=60,
        ) {
            const HEX: &[u8] = b"0123456789abcdefABCDEF";
            let int_s: String = int_digits.iter().map(|&i| HEX[i % HEX.len()] as char).collect();
            let frac_s: String = frac_digits.iter().map(|&i| HEX[i % HEX.len()] as char).collect();
            let lit = if frac_s.is_empty() {
                format!("0x{int_s}p{exp}")
            } else {
                format!("0x{int_s}.{frac_s}p{exp}")
            };
            let mant = u64::from_str_radix(&format!("{int_s}{frac_s}"), 16).unwrap();
            let expected = (mant as f64) * 2f64.powi(exp - 4 * frac_s.len() as i32);
            let k = first_tok(&lit);
            let got = float_payload(&k);
            prop_assert_eq!(got.to_bits(), expected.to_bits(), "lit={:?}", lit);
        }
    }

    // P6 ─ hex float must not collapse to 0 / wrong value on wide inputs ────
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p6_hex_float_no_collapse(
            int_digits in prop::collection::vec(0usize..16, 1..20),
            exp in -1000i32..=1000,
        ) {
            const HEX: &[u8] = b"0123456789abcdefABCDEF";
            let int_s: String = int_digits.iter().map(|&i| HEX[i % HEX.len()] as char).collect();
            prop_assume!(int_s.chars().any(|c| c != '0'));
            let lit = format!("0x{int_s}p{exp}");
            let got = float_payload(&first_tok(&lit));
            prop_assert!(!got.is_nan(), "lit={:?}", lit);
            prop_assert_ne!(got, 0.0, "nonzero literal collapsed to 0.0: lit={:?}", lit);
        }
    }

    #[test]
    fn p6b_hex_float_huge_exponent() {
        // Positive huge exponents: true value overflows double -> must be infinite (or at least huge), never 1.0
        for e in ["4294967296", "2147483648", "18446744073709551616", "99999999999999999999"] {
            let lit = format!("0x1p{e}");
            let v = float_payload(&first_tok(&lit));
            assert!(v.is_infinite() || v >= 1e300, "lit={lit:?} gave {v}");
        }
        // Negative huge exponents: true value underflows to exactly 0.0, never 1.0
        for e in ["-4294967296", "-2147483648", "-18446744073709551616", "-99999999999999999999"] {
            let lit = format!("0x1p{e}");
            let v = float_payload(&first_tok(&lit));
            assert_eq!(v, 0.0, "lit={lit:?} gave {v}");
        }
    }

    // P7 ─ integer literal overflow: payload ≡ digits (mod 2^64) ────────────
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p7_int_overflow_wraparound(
            digits in prop::collection::vec(0usize..16, 1..20),
            base in 0usize..3,
        ) {
            const HEX: &[u8] = b"0123456789abcdefABCDEF";
            const OCT: &[u8] = b"01234567";
            const BIN: &[u8] = b"01";
            let (prefix, radix): (&str, u32) = match base {
                0 => ("0x", 16),
                1 => ("0", 8),
                _ => ("0b", 2),
            };
            let digits_s: String = digits
                .iter()
                .map(|&i| {
                    let alpha = match radix { 16 => HEX, 8 => OCT, _ => BIN };
                    alpha[i % alpha.len()] as char
                })
                .collect();
            let text = format!("{prefix}{digits_s}");
            let expected = u128::from_str_radix(&digits_s, radix).unwrap() as u64;
            let got = int_payload(&first_tok(&text));
            prop_assert_eq!(got, expected, "text={:?}", text);
        }
    }

    #[test]
    fn p7b_decimal_exact_up_to_u64() {
        for d in ["0", "9", "2147483647", "2147483648", "4294967296",
                  "9223372036854775807", "18446744073709551615"] {
            let got = int_payload(&first_tok(d));
            assert_eq!(got, d.parse::<u64>().unwrap(), "decimal {d:?}");
        }
    }

    // P8 ─ escape sequences vs documented table (README:275-283, C11 6.4.4.4)
    #[test]
    fn p8a_simple_escapes_table() {
        let table: &[(&str, char)] = &[
            ("n", '\n'), ("t", '\t'), ("r", '\r'), ("\\", '\\'),
            ("'", '\''), ("\"", '"'), ("a", '\x07'), ("b", '\x08'),
            ("e", '\x1b'), ("E", '\x1b'), ("f", '\x0c'), ("v", '\x0b'),
        ];
        for (esc, want) in table {
            let lit = format!("'\\{esc}'");
            assert_eq!(first_tok(&lit), TokenKind::CharLiteral(*want), "esc=\\{esc}");
            let slit = format!("\"\\{esc}\"");
            assert_eq!(first_tok(&slit), TokenKind::StringLiteral(want.to_string()));
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p8b_octal_hex_escapes(d in 0u32..512) {
            // octal escape: 1-3 digits, value truncated to byte (README:278)
            let oct = format!("{:o}", d);
            let lit = format!("'\\{oct}'");
            match first_tok(&lit) {
                TokenKind::CharLiteral(c) => {
                    prop_assert_eq!(c as u32, d & 0xFF, "octal lit={:?}", lit);
                }
                other => panic!("octal escape {lit:?} not a CharLiteral: {other:?}"),
            }
            // hex escape: consumes all digits, truncated to byte (README:279)
            let hex = format!("{:x}", d);
            let hlit = format!("'\\x{hex}'");
            match first_tok(&hlit) {
                TokenKind::CharLiteral(c) => {
                    prop_assert_eq!(c as u32, d & 0xFF, "hex lit={:?}", hlit);
                }
                other => panic!("hex escape {hlit:?} not a CharLiteral: {other:?}"),
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p8c_unicode_escapes(cp_raw in 0u32..=0x10FFFF) {
            let Some(cp) = char::from_u32(cp_raw) else {
                return Ok(()); // surrogates have no char; U+FFFD path tested in T-series
            };
            let escs = if (cp as u32) <= 0xFFFF {
                vec![format!("\\u{:04X}", cp as u32), format!("\\U{:08X}", cp as u32)]
            } else {
                vec![format!("\\U{:08X}", cp as u32)]
            };
            // wide char literal: code point directly (README:266)
            for esc in &escs {
                let lit = format!("L'{esc}'");
                prop_assert_eq!(first_tok(&lit), TokenKind::IntLiteral(cp as i64), "wide lit={:?}", lit);
            }
            // narrow char literal: <=0xFF single char; >0xFF UTF-8-packed int (README:258)
            let lit = format!("'{}'", escs[0]);
            let k = first_tok(&lit);
            if (cp as u32) <= 0xFF {
                prop_assert_eq!(k, TokenKind::CharLiteral(cp), "narrow lit={:?}", lit);
            } else {
                let mut packed: i32 = 0;
                let mut buf = [0u8; 4];
                let utf8 = cp.encode_utf8(&mut buf);
                for b in utf8.bytes() {
                    packed = (packed << 8) | b as i32;
                }
                // multichar constants have type int (C11 6.4.4.4p10); value is the
                // 32-bit packed pattern, sign-extended when widened to i64
                prop_assert_eq!(k, TokenKind::IntLiteral(packed as i64), "narrow packed lit={:?}", lit);
            }
            // narrow string literal: UTF-8 encoded byte-by-byte (README:248)
            let slit = format!("\"{}\"", escs[escs.len() - 1]);
            let mut want = String::new();
            let mut buf = [0u8; 4];
            let utf8 = cp.encode_utf8(&mut buf);
            for b in utf8.bytes() {
                want.push(b as char);
            }
            prop_assert_eq!(first_tok(&slit), TokenKind::StringLiteral(want), "string lit={:?}", slit);
        }
    }

    #[test]
    fn p8d_multichar_packing() {
        assert_eq!(first_tok("'AB'"), TokenKind::IntLiteral(0x4142));
        assert_eq!(first_tok("'abcd'"), TokenKind::IntLiteral(0x61626364));
        assert_eq!(first_tok("L'x'"), TokenKind::IntLiteral('x' as i64));
    }

    // P9 ─ whitespace/comment insertion is token-stream invariant ───────────
    const SEPS: &[&str] = &[" ", "\t", "\n", "  \t ", "\n\n", "/*xy*/", "//c\n", " /*a*/\n"];

    fn safe_token_strategy() -> BoxedStrategy<(String, TokenKind)> {
        prop_oneof![
            Just(("x".into(), TokenKind::Identifier("x".into()))),
            Just(("foo42".into(), TokenKind::Identifier("foo42".into()))),
            (0i64..i32::MAX as i64)
                .prop_map(|v| (v.to_string(), TokenKind::IntLiteral(v))),
            Just(("+".into(), TokenKind::Plus)),
            Just(("-".into(), TokenKind::Minus)),
            Just(("*".into(), TokenKind::Star)),
            Just(("(".into(), TokenKind::LParen)),
            Just((")".into(), TokenKind::RParen)),
            Just((";".into(), TokenKind::Semicolon)),
            Just((",".into(), TokenKind::Comma)),
            Just((".".into(), TokenKind::Dot)),
            Just(("<".into(), TokenKind::Less)),
            Just((">".into(), TokenKind::Greater)),
            Just(("==".into(), TokenKind::EqualEqual)),
            (any::<char>().prop_map(|c| (format!("'{}'", c), TokenKind::CharLiteral(c))))
                .prop_filter("alnum only", |(_, k)| matches!(k, TokenKind::CharLiteral(c) if c.is_ascii_alphanumeric())),
            (prop::collection::vec(any::<char>(), 1..8)).prop_map(|cs| {
                let body: String = cs.iter().filter(|c| c.is_ascii_alphabetic()).collect();
                let b = if body.is_empty() { "a".to_string() } else { body };
                (format!("\"{b}\""), TokenKind::StringLiteral(b))
            }),
        ]
            .boxed()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p9_whitespace_invariance(
            toks in prop::collection::vec(safe_token_strategy(), 1..8),
            seeds in prop::collection::vec(any::<u64>(), 1..16),
        ) {
            let render = |seed_shift: u64| -> String {
                let mut out = String::new();
                for (i, (text, _)) in toks.iter().enumerate() {
                    if i > 0 {
                        let idx = (seeds[i % seeds.len()].wrapping_add(seed_shift)) as usize % SEPS.len();
                        out.push_str(SEPS[idx]);
                    }
                    out.push_str(text);
                }
                out
            };
            let a = kinds_of(&render(0));
            let b = kinds_of(&render(7));
            let mut expected: Vec<TokenKind> = toks.iter().map(|(_, k)| k.clone()).collect();
            expected.push(TokenKind::Eof);
            prop_assert_eq!(a, expected.clone(), "render A: {:?}", render(0));
            prop_assert_eq!(b, expected, "render B: {:?}", render(7));
        }
    }

    // P10 ─ arbitrary input: terminate, end with Eof, spans sane ────────────
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p10_fuzz_spans_and_termination(s in prop::collection::vec(any::<char>(), 0..64)) {
            let input: String = s.into_iter().collect();
            let toks = Lexer::new(&input, 0).tokenize();
            let last = toks.last().expect("tokenize must emit at least Eof");
            prop_assert!(last.is_eof());
            prop_assert_eq!(last.span.start as usize, input.len());
            prop_assert_eq!(last.span.end as usize, input.len());
            for w in toks.windows(2) {
                prop_assert!(w[1].span.start >= w[0].span.end,
                    "span regression at {}..{} -> {}..{} in {input:?}",
                    w[0].span.start, w[0].span.end, w[1].span.start, w[1].span.end);
            }
            for t in &toks {
                prop_assert!(t.span.start <= t.span.end);
                prop_assert!(t.span.end as usize <= input.len());
            }
        }
    }

    // KAT gate for reference oracles (P2/P4/P5/P8) — must pass before PBT ran
    #[test]
    fn kat_reference_gate() {
        assert_eq!(float_payload(&first_tok("0x1.8p1")), 3.0);
        assert_eq!(float_payload(&first_tok("0x10p2")), 64.0);
        assert_eq!(float_payload(&first_tok("0x1p-2")), 0.25);
        assert_eq!(int_payload(&first_tok("0xff")), 255);
        assert_eq!(first_tok("int"), TokenKind::Int);
        assert_eq!(first_tok("'A'"), TokenKind::CharLiteral('A'));
    }

    // P11 (sweep round) ─ documented integer/float suffix grammar incl.
    // imaginary i/I/j/J combinations (README:213-217, 245-251)
    #[test]
    fn p11_imaginary_suffix_combinations() {
        // integer imaginary: 5i / 5I / 5j / 5J / 5ui / 5li / 5lli / 5ulli / 5ULi
        for (lit, want) in [
            ("5i", TokenKind::ImaginaryLiteral(5.0)),
            ("5I", TokenKind::ImaginaryLiteral(5.0)),
            ("5j", TokenKind::ImaginaryLiteral(5.0)),
            ("5J", TokenKind::ImaginaryLiteral(5.0)),
            ("5ui", TokenKind::ImaginaryLiteral(5.0)),
            ("5li", TokenKind::ImaginaryLiteral(5.0)),
            ("5lli", TokenKind::ImaginaryLiteral(5.0)),
            ("5ulli", TokenKind::ImaginaryLiteral(5.0)),
            ("0x10i", TokenKind::ImaginaryLiteral(16.0)),
        ] {
            assert_eq!(first_tok(lit), want, "int imaginary suffix {lit:?}");
        }
        // float imaginary: kind 0=double, 1=float, 2=long double (README:245-251);
        // i/I may appear before or after the type suffix; j/J after or standalone
        for (lit, want_kind, want_val) in [
            ("1.5i", 0u8, 1.5f64),
            ("1.5j", 0, 1.5),
            ("1.5J", 0, 1.5),
            ("1.5fi", 1, 1.5),
            ("1.5if", 1, 1.5),
            ("1.5fj", 1, 1.5),
            ("1.5Fi", 1, 1.5),
            ("2.5Li", 2, 2.5),
            ("2.5iL", 2, 2.5),
            ("2.5Lj", 2, 2.5),
        ] {
            match first_tok(lit) {
                TokenKind::ImaginaryLiteral(v) => {
                    assert_eq!(want_kind, 0, "wrong kind for {lit:?}");
                    assert_eq!(v, want_val, "{lit:?}");
                }
                TokenKind::ImaginaryLiteralF32(v) => {
                    assert_eq!(want_kind, 1, "wrong kind for {lit:?}");
                    assert_eq!(v, want_val, "{lit:?}");
                }
                TokenKind::ImaginaryLiteralLongDouble(v, _) => {
                    assert_eq!(want_kind, 2, "wrong kind for {lit:?}");
                    assert_eq!(v, want_val, "{lit:?}");
                }
                k => panic!("{lit:?} -> {k:?}"),
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p11b_random_int_suffix_round_trip(
            v in 0u64..0xFFFF,
            us in 0usize..3, ls in 0usize..4, tail in 0usize..2,
        ) {
            // README:213: suffix order is flexible; u/U and l/L repeatable (GCC warns
            // for >2 but still lexes); trailing i/j (if next char is not ident-continuation).
            let mut suf = String::new();
            for _ in 0..us { suf.push('u'); }
            for _ in 0..ls { suf.push('l'); }
            if tail == 1 { suf.push('j'); }
            let lit = format!("{v}{suf}");
            let k = first_tok(&lit);
            if tail == 1 {
                prop_assert!(matches!(k, TokenKind::ImaginaryLiteral(_)), "lit={:?} -> {:?}", lit, k);
            } else {
                let unsigned = us > 0;
                let long_long = ls >= 2;
                let long_only = ls == 1;
                let expected = match (unsigned, long_long, long_only) {
                    (true, true, _) => TokenKind::ULongLongLiteral(v),
                    (true, false, true) => TokenKind::ULongLiteral(v),
                    (true, false, false) => {
                        if v <= u32::MAX as u64 { TokenKind::UIntLiteral(v) } else { TokenKind::ULongLiteral(v) }
                    }
                    (false, true, _) => TokenKind::LongLongLiteral(v as i64),
                    (false, false, true) => TokenKind::LongLiteral(v as i64),
                    (false, false, false) => TokenKind::IntLiteral(v as i64),
                };
                prop_assert_eq!(k, expected, "lit={:?}", lit);
            }
        }
    }

    // P12 (sweep round) ─ surrogate U+FFFD fallback (README:283) + wide/u8
    // string code-point storage (README:248-254)
    #[test]
    fn p12_surrogate_fallback_and_string_families() {
        // Surrogate code points have no char: documented fallback U+FFFD
        assert_eq!(first_tok("'\\ud800'"), TokenKind::IntLiteral(0xEFBFBD),
            "narrow surrogate: U+FFFD UTF-8-packed");
        assert_eq!(first_tok("L'\\ud800'"), TokenKind::IntLiteral(0xFFFD),
            "wide surrogate: code point U+FFFD directly");
        // max code point packs to F4 8F BF BF; multichar constants have type int
        // (C11 6.4.4.4p10) so the high bit makes the value negative
        assert_eq!(first_tok("'\\U0010FFFF'"),
            TokenKind::IntLiteral(0xF48FBFBFu32 as i32 as i64), "narrow max code point packed");
        // Wide strings store code points directly (not UTF-8 bytes)
        assert_eq!(first_tok("L\"\\u00e9\""), TokenKind::WideStringLiteral("\u{00e9}".into()));
        // u8"..." is lexed identically to a narrow string (README:250)
        assert_eq!(first_tok("u8\"\\u00e9\""), first_tok("\"\\u00e9\""));
        // u"..." is Char16StringLiteral
        assert_eq!(first_tok("u\"ab\""), TokenKind::Char16StringLiteral("ab".into()));
        // u'x' uses the wide-char path (code point as IntLiteral)
        assert_eq!(first_tok("u'x'"), TokenKind::IntLiteral('x' as i64));
        assert_eq!(first_tok("u8'x'"), TokenKind::IntLiteral('x' as i64));
    }
}

#[cfg(test)]
mod pbt_regression {
    use super::Lexer;
    use crate::frontend::lexer::token::TokenKind;

    fn kinds(input: &str) -> Vec<TokenKind> {
        Lexer::new(input, 0).tokenize().into_iter().map(|t| t.kind).collect()
    }

    // T1: documented line-marker handling (README:162)
    #[test]
    fn t1_line_markers() {
        assert_eq!(
            kinds("# 42 \"f.c\"\nint x;"),
            vec![TokenKind::Int, TokenKind::Identifier("x".into()), TokenKind::Semicolon, TokenKind::Eof]
        );
        // '#' not at start of line stays a Hash token
        assert_eq!(
            kinds("x # 3"),
            vec![TokenKind::Identifier("x".into()), TokenKind::Hash, TokenKind::IntLiteral(3), TokenKind::Eof]
        );
        // '#if' is not a line marker (no digit)
        assert_eq!(
            kinds("#if 0"),
            vec![TokenKind::Hash, TokenKind::If, TokenKind::IntLiteral(0), TokenKind::Eof]
        );
    }

    // T2: unterminated comment — BUG WITNESS (B5): the last byte of an
    // unterminated block comment leaks out as a token (scan.rs block-comment
    // loop stops at pos+1 < len, leaving the final byte unconsumed).
    #[test]
    fn t2_unterminated_comment() {
        assert_eq!(kinds("/*"), vec![TokenKind::Eof]);
        assert_eq!(
            kinds("int /* gone"),
            vec![TokenKind::Int, TokenKind::Eof]
        );
    }

    // B4 probe: runs of unknown non-ASCII chars must terminate. lex_punctuation
    // recurses per character (unknown-char branch), so this stays below the
    // observed stack-overflow threshold (~2000-4000 chars on a 2 MiB thread stack).
    #[test]
    fn probe_unknown_char_stack_depth_safe() {
        for n in [1usize, 10, 100, 500, 1000, 2000] {
            let s: String = "\u{00ff}".repeat(n);
            let toks = Lexer::new(&s, 0).tokenize();
            assert!(toks.last().unwrap().is_eof(), "n={n}");
        }
    }

    // B4 witness: at ~4000+ unknown chars the recursive unknown-char path
    // overflows the stack and ABORTS the process. Run in isolation:
    //   cargo test --lib frontend::lexer::scan::pbt_regression::probe_unknown_char_stack_crash -- --ignored --exact
    #[test]
    #[ignore = "B4 witness: aborts the test process (stack overflow, SIGABRT)"]
    fn probe_unknown_char_stack_crash() {
        let s: String = "\u{00ff}".repeat(200_000);
        let toks = Lexer::new(&s, 0).tokenize();
        assert!(toks.last().unwrap().is_eof());
    }

    // ── deterministic regression tests (shrunk witnesses; FAIL until fixed) ──

    fn tok0(input: &str) -> TokenKind {
        Lexer::new(input, 0).tokenize().remove(0).kind
    }

    // B1: 0x10000000000000000p0 == 2^64 — exactly representable f64, valid C99
    // hex float; GCC accepts it without complaint. ccc evaluates it to 0.0.
    #[test]
    fn test_lex_hex_float_regression_wide_mantissa_zero() {
        assert_eq!(tok0("0x10000000000000000p0"), TokenKind::FloatLiteral(18446744073709551616.0));
    }

    // B2: exponent 2^32 truncates to i32 0 -> 1.0 instead of inf; -2^32 -> 1.0 instead of 0.0.
    #[test]
    fn test_lex_hex_float_regression_exponent_truncation() {
        match tok0("0x1p4294967296") {
            TokenKind::FloatLiteral(v) => assert!(v.is_infinite(), "got {v}"),
            k => panic!("not a float: {k:?}"),
        }
        match tok0("0x1p-4294967296") {
            TokenKind::FloatLiteral(v) => assert_eq!(v, 0.0),
            k => panic!("not a float: {k:?}"),
        }
    }

    // B3: 17-hex-digit integer: true value 2^64+1, mod 2^64 = 1 (GCC: warns, value 1);
    // ccc silently yields 0.
    #[test]
    fn test_lex_int_regression_u64_overflow_zero() {
        assert_eq!(tok0("0x10000000000000001"), TokenKind::IntLiteral(1));
    }

    // B5: unterminated block comment leaks its LAST byte as a token.
    #[test]
    fn test_lex_comment_regression_unterminated_leaks_last_byte() {
        assert_eq!(
            Lexer::new("int /* gone", 0).tokenize().into_iter().map(|t| t.kind).collect::<Vec<_>>(),
            vec![TokenKind::Int, TokenKind::Eof]
        );
    }

    // T3: synthetic pragma tokens (documented formats)
    #[test]
    fn t3_pragma_synthetic_tokens() {
        assert_eq!(kinds("__ccc_pack_set_16")[0], TokenKind::PragmaPackSet(16));
        assert_eq!(kinds("__ccc_pack_pop")[0], TokenKind::PragmaPackPop);
        assert_eq!(kinds("__ccc_visibility_push_hidden")[0],
            TokenKind::PragmaVisibilityPush("hidden".into()));
    }

    // T4: octal + ellipsis special case (README:205)
    #[test]
    fn t4_octal_ellipsis() {
        assert_eq!(
            kinds("case 0 ... 5 :"),
            vec![TokenKind::Case, TokenKind::IntLiteral(0), TokenKind::Ellipsis,
                  TokenKind::IntLiteral(5), TokenKind::Colon, TokenKind::Eof]
        );
        assert_eq!(
            kinds("07..."),
            vec![TokenKind::IntLiteral(7), TokenKind::Ellipsis, TokenKind::Eof]
        );
    }

    // B1/B2/B3 witnesses — filled during Review from shrunk counterexamples.
}

