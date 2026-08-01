use crate::token::{Token, TokenKind};

pub fn emit_stripped(tokens: &[Token]) -> String {
    let mut out = String::new();
    for tok in tokens {
        match tok.kind {
            TokenKind::Code | TokenKind::StringLiteral | TokenKind::Newline => {
                out.push_str(&tok.text);
            }

            TokenKind::LineComment => {}

            TokenKind::BlockComment => {
                out.push(' ');
            }
        }
    }
    out
}

pub fn emit_list(tokens: &[Token]) {
    let mut line = 1usize;
    for tok in tokens {
        match &tok.kind {
            TokenKind::Newline => line += 1,
            TokenKind::LineComment => {
                let body = tok.text.trim();
                println!("  line {:>4}  {}", line, body);
            }
            TokenKind::BlockComment => {
                let lines: Vec<&str> = tok
                    .text
                    .lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .collect();
                let preview = lines.first().copied().unwrap_or("(empty)");
                let ellipsis = if lines.len() > 1 { " …" } else { "" };
                println!("  line {:>4}  /* {}{} */", line, preview, ellipsis);
            }
            _ => {}
        }
    }
}

pub fn emit_count(tokens: &[Token]) -> (usize, usize) {
    let (mut lc, mut bc) = (0usize, 0usize);
    for tok in tokens {
        match tok.kind {
            TokenKind::LineComment => lc += 1,
            TokenKind::BlockComment => bc += 1,
            _ => {}
        }
    }
    (lc, bc)
}
