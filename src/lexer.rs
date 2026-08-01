use crate::rules::LanguageRules;
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a [char],
    pos: usize,
    rules: &'a LanguageRules,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a [char], rules: &'a LanguageRules) -> Self {
        Self { src, pos: 0, rules }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.src.len() {
            tokens.push(self.next_token());
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        if self.src[self.pos] == '\n' {
            self.pos += 1;
            return Token::new(TokenKind::Newline, "\n");
        }

        if self.rules.raw_strings {
            if let Some(tok) = self.try_rust_raw_string() {
                return tok;
            }
        }

        let mut block_openers: Vec<&[String; 2]> = self.rules.block_comments.iter().collect();
        block_openers.sort_by(|a, b| b[0].len().cmp(&a[0].len()));
        for pair in block_openers {
            if self.peek_str(&pair[0]) {
                return self.consume_block_comment(&pair[0], &pair[1]);
            }
        }

        let mut line_prefixes: Vec<&String> = self.rules.line_comments.iter().collect();
        line_prefixes.sort_by(|a, b| b.len().cmp(&a.len()));
        for prefix in line_prefixes {
            if self.peek_str(prefix) {
                return self.consume_line_comment(prefix.len());
            }
        }

        let mut string_delims: Vec<usize> = (0..self.rules.strings.len()).collect();
        string_delims.sort_by(|&a, &b| {
            self.rules.strings[b]
                .open
                .len()
                .cmp(&self.rules.strings[a].open.len())
        });
        for idx in string_delims {
            let delim = &self.rules.strings[idx];
            if self.peek_str(&delim.open) {
                if delim.char_literal && !self.looks_like_char_literal() {
                    break;
                }
                let (open, close, escape, multiline) = (
                    delim.open.clone(),
                    delim.close.clone(),
                    delim.escape,
                    delim.multiline,
                );
                return self.consume_string(&open, &close, escape, multiline);
            }
        }

        let ch = self.src[self.pos];
        self.pos += 1;
        Token::new(TokenKind::Code, ch.to_string())
    }

    fn consume_line_comment(&mut self, prefix_len: usize) -> Token {
        self.pos += prefix_len;
        let mut text = String::new();
        while self.pos < self.src.len() && self.src[self.pos] != '\n' {
            text.push(self.src[self.pos]);
            self.pos += 1;
        }
        Token::new(TokenKind::LineComment, text)
    }

    fn consume_block_comment(&mut self, open: &str, close: &str) -> Token {
        self.pos += open.chars().count();
        let mut text = String::new();
        while self.pos < self.src.len() {
            if self.peek_str(close) {
                self.pos += close.chars().count();
                return Token::new(TokenKind::BlockComment, text);
            }
            text.push(self.src[self.pos]);
            self.pos += 1;
        }
        Token::new(TokenKind::BlockComment, text)
    }

    fn consume_string(&mut self, open: &str, close: &str, escape: bool, multiline: bool) -> Token {
        self.pos += open.chars().count();
        let mut text = open.to_owned();
        while self.pos < self.src.len() {
            if escape && self.src[self.pos] == '\\' && self.pos + 1 < self.src.len() {
                text.push(self.src[self.pos]);
                text.push(self.src[self.pos + 1]);
                self.pos += 2;
                continue;
            }
            if self.peek_str(close) {
                text.push_str(close);
                self.pos += close.chars().count();
                return Token::new(TokenKind::StringLiteral, text);
            }
            if !multiline && self.src[self.pos] == '\n' {
                return Token::new(TokenKind::StringLiteral, text);
            }
            text.push(self.src[self.pos]);
            self.pos += 1;
        }
        Token::new(TokenKind::StringLiteral, text)
    }

    fn try_rust_raw_string(&mut self) -> Option<Token> {
        let start = self.pos;
        let len = self.src.len();

        if self.src[start] != 'r' {
            return None;
        }

        let mut hashes = 0usize;
        while start + 1 + hashes < len && self.src[start + 1 + hashes] == '#' {
            hashes += 1;
        }

        let quote_pos = start + 1 + hashes;
        if quote_pos >= len || self.src[quote_pos] != '"' {
            return None;
        }

        self.pos = quote_pos + 1;
        let mut text: String = self.src[start..self.pos].iter().collect();

        loop {
            if self.pos >= len {
                return Some(Token::new(TokenKind::StringLiteral, text));
            }
            if self.src[self.pos] == '"' {
                let close_start = self.pos;
                let mut h = 0usize;
                while close_start + 1 + h < len
                    && h < hashes
                    && self.src[close_start + 1 + h] == '#'
                {
                    h += 1;
                }
                if h == hashes {
                    self.pos = close_start + 1 + hashes;
                    let close_slice: String = self.src[close_start..self.pos].iter().collect();
                    text.push_str(&close_slice);
                    return Some(Token::new(TokenKind::StringLiteral, text));
                }
            }
            text.push(self.src[self.pos]);
            self.pos += 1;
        }
    }

    fn looks_like_char_literal(&self) -> bool {
        let start = self.pos + 1;
        let mut i = start;
        let len = self.src.len();

        if i >= len {
            return false;
        }

        if self.src[i] == '\\' {
            i += 1;
            if i >= len {
                return false;
            }
            if self.src[i] == 'u' && i + 1 < len && self.src[i + 1] == '{' {
                i += 2;
                while i < len && self.src[i] != '}' && self.src[i] != '\n' {
                    i += 1;
                }
                if i >= len || self.src[i] != '}' {
                    return false;
                }
                i += 1;
            } else {
                i += 1;
            }
        } else if self.src[i] == '\n' || self.src[i] == '\'' {
            return false;
        } else {
            i += 1;
        }

        i < len && self.src[i] == '\''
    }

    fn peek_str(&self, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if self.pos + chars.len() > self.src.len() {
            return false;
        }
        self.src[self.pos..self.pos + chars.len()] == chars[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{LanguageRules, StringDelimiter};
    use crate::token::TokenKind;

    fn rust_rules() -> LanguageRules {
        LanguageRules {
            name: "Rust".into(),
            extensions: vec!["rs".into()],
            line_comments: vec!["//".into()],
            block_comments: vec![["/*".into(), "*/".into()]],
            raw_strings: true,
            strings: vec![
                StringDelimiter {
                    open: "\"".into(),
                    close: "\"".into(),
                    escape: true,
                    multiline: false,
                    char_literal: false,
                },
                StringDelimiter {
                    open: "'".into(),
                    close: "'".into(),
                    escape: true,
                    multiline: false,
                    char_literal: true,
                },
            ],
        }
    }

    fn lex(src: &str, rules: &LanguageRules) -> Vec<Token> {
        let chars: Vec<char> = src.chars().collect();
        Lexer::new(&chars, rules).tokenize()
    }

    fn kinds(tokens: &[Token]) -> Vec<TokenKind> {
        tokens.iter().map(|t| t.kind.clone()).collect()
    }

    fn strip(src: &str, rules: &LanguageRules) -> String {
        crate::emitter::emit_stripped(&lex(src, rules))
    }

    #[test]
    fn comment_in_string_is_not_stripped() {
        let src = r#"let s = "hello // world";"#;
        let tokens = lex(src, &rust_rules());
        assert!(!kinds(&tokens).contains(&TokenKind::LineComment));
    }

    #[test]
    fn line_comment_produces_token() {
        let src = "let x = 1; // assign\nlet y = 2;";
        let tokens = lex(src, &rust_rules());
        assert!(kinds(&tokens).contains(&TokenKind::LineComment));
    }

    #[test]
    fn block_comment_produces_token() {
        let src = "a /* block */ b";
        let tokens = lex(src, &rust_rules());
        assert!(kinds(&tokens).contains(&TokenKind::BlockComment));
    }

    #[test]
    fn newline_after_line_comment_is_preserved() {
        let src = "a // comment\nb";
        let tokens = lex(src, &rust_rules());
        let ks = kinds(&tokens);
        let comment_pos = ks
            .iter()
            .position(|k| k == &TokenKind::LineComment)
            .unwrap();
        assert_eq!(ks[comment_pos + 1], TokenKind::Newline);
    }

    #[test]
    fn escaped_quote_does_not_end_string() {
        let src = r#"let s = "he said \"hi\"";"#;
        let tokens = lex(src, &rust_rules());
        let string_count = kinds(&tokens)
            .iter()
            .filter(|k| k == &&TokenKind::StringLiteral)
            .count();
        assert_eq!(string_count, 1);
    }

    #[test]
    fn raw_string_hides_double_slash() {
        let src = r####"let s = r#"hello // world"#;"####;
        let tokens = lex(src, &rust_rules());
        assert!(
            !kinds(&tokens).contains(&TokenKind::LineComment),
            "// inside raw string should not become a comment"
        );
    }

    #[test]
    fn raw_string_hides_block_comment() {
        let src = r####"let s = r#"/* not a comment */"#;"####;
        let tokens = lex(src, &rust_rules());
        assert!(
            !kinds(&tokens).contains(&TokenKind::BlockComment),
            "/* */ inside raw string should not become a block comment"
        );
    }

    #[test]
    fn raw_string_no_hashes() {
        let src = "let s = r\"hello // world\"; // real";
        let tokens = lex(src, &rust_rules());
        let comments: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::LineComment)
            .collect();
        assert_eq!(
            comments.len(),
            1,
            "only the real comment after the raw string should be found"
        );
    }

    #[test]
    fn raw_string_multiple_hashes() {
        let src = r#####"let s = r##"has "one" hash and r#"nested"# inside"##; // real"#####;
        let tokens = lex(src, &rust_rules());
        let comments: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::LineComment)
            .collect();
        assert_eq!(comments.len(), 1, "only one real comment expected");
    }

    #[test]
    fn raw_string_content_preserved_verbatim() {
        let src = "let s = r\"C:\\Users\\foo\"; // strip";
        let result = strip(src, &rust_rules());
        assert!(
            result.contains(r"C:\Users\foo"),
            "raw string content must be verbatim"
        );
        assert!(
            !result.contains("strip"),
            "comment after raw string must be removed"
        );
    }

    #[test]
    fn lifetime_annotation_does_not_eat_comment() {
        let src = "fn foo<'a>(s: &'a str) -> &'a str { // comment\n    s\n}";
        let result = strip(src, &rust_rules());
        assert!(
            !result.contains("comment"),
            "comment after lifetime annotation must be stripped"
        );
        assert!(
            result.contains("'a"),
            "lifetime annotation must be preserved"
        );
    }

    #[test]
    fn char_literals_preserved() {
        let src = "let a = 'x'; // strip\nlet b = '\\n'; // strip\nlet c = '\\''; // strip\n";
        let result = strip(src, &rust_rules());
        assert!(result.contains("'x'"));
        assert!(result.contains("'\\n'"));
        assert!(result.contains("'\\''"));
        assert!(!result.contains("strip"));
    }
}
