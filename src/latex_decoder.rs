use unicode_normalization::UnicodeNormalization;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    #[regex(r"[^\{}]", |lex| lex.slice())]
    Char(&'a str),

    #[regex("[{}]")]
    Brace,

    #[regex(r#"\\`[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0300}'))]
    Grave(String),

    #[regex(r#"\\'[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0301}'))]
    Acute(String),

    #[regex(r#"\\\^[\{\\]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0302}'))]
    Circumflex(String),

    #[regex(r#"\\"[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0308}'))]
    Umlaut(String),

    #[regex(r#"\\H[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{030B}'))]
    HungarianUmlaut(String),

    #[regex(r#"\\~[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0303}'))]
    Tilde(String),

    #[regex(r#"\\c[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0327}'))]
    Cedilla(String),

    #[regex(r#"\\k[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0328}'))]
    Ogonek(String),

    #[regex(r#"\\l"#, |_| String::from("\u{0142}"))]
    Barredl(String),

    #[regex(r#"\\=[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0304}'))]
    Macron(String),

    #[regex(r#"\\b[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0332}'))]
    BarUnder(String),

    #[regex(r#"\\\.[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0307}'))]
    DotOver(String),

    #[regex(r#"\\d[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0323}'))]
    DotUnder(String),

    #[regex(r#"\\r[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{030A}'))]
    RingOver(String),

    #[regex(r#"\\u[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0306}'))]
    Breve(String),

    #[regex(r#"\\v[\{\\}]*[a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{030C}'))]
    Caron(String),

    #[regex(r#"\\t[\{\\}]*[a-zA-Z][a-zA-Z]"#, |lex| add_diacritic(lex.slice(), '\u{0308}'))]
    Tie(String),

    #[regex(r#"\\\{*o"#, |_| String::from("\u{00F8}"))]
    SlashedO(String),

    #[regex(r#"\\aa"#, |_| String::from("å"))]
    RingOverA(String),
    
    #[regex(r#"\\."#, |lex| String::from(&lex.slice()[1..]))]
    Backslash(String),

    #[error]
    Error,
}

fn add_diacritic<'c, 'd>(slice: &str, combiner: char) -> String {
    let mut result = String::new();
    result.push_str(&slice[slice.len() - 1..slice.len()]);
    result.push(combiner);
    return result
}

pub fn decode_latex(latex: &str) -> String {
    let mut result = String::new();
    let mut lex = Token::lexer(latex);
    loop {
        match lex.next() {
            Some(Token::Char(c)) => result.push_str(c),
            Some(Token::Brace) => (),
            // Some(Token::Combiner(comb)) => (),
            Some(Token::Grave(s)) => result.push_str(&s),
            Some(Token::Acute(s)) => result.push_str(&s),
            Some(Token::Circumflex(s)) => result.push_str(&s),
            Some(Token::Umlaut(s)) => result.push_str(&s),
            Some(Token::HungarianUmlaut(s)) => result.push_str(&s),
            Some(Token::Tilde(s)) => result.push_str(&s),
            Some(Token::Cedilla(s)) => result.push_str(&s),
            Some(Token::Ogonek(s)) => result.push_str(&s),
            Some(Token::Barredl(s)) => result.push_str(&s),
            Some(Token::Macron(s)) => result.push_str(&s),
            Some(Token::BarUnder(s)) => result.push_str(&s),
            Some(Token::DotOver(s)) => result.push_str(&s),
            Some(Token::DotUnder(s)) => result.push_str(&s),
            Some(Token::RingOver(s)) => result.push_str(&s),
            Some(Token::RingOverA(s)) => result.push_str(&s),
            Some(Token::Breve(s)) => result.push_str(&s),
            Some(Token::Caron(s)) => result.push_str(&s),
            Some(Token::Tie(s)) => result.push_str(&s),
            Some(Token::SlashedO(s)) => result.push_str(&s),
            // Some(Token::Umlaut(s)) => result.push_str(&s),
            // Some(Token::Command(command)) => (),
            Some(Token::Backslash(s)) => result.push_str(&s),
            Some(Token::Error) => (),
            None => break
        }
    }

    return result.nfc().collect::<String>();
}

#[test]
fn decode_plain_ascii_text() {
    let input = "Hello World! 123 _";
    let result = decode_latex(input);
    assert_eq!(result, input)
}

#[test]
fn decode_plain_text_with_whitespace() {
    let input = "Hello World!\nHello Other World";
    let result = decode_latex(input);
    assert_eq!(result, input)
}

#[test]
fn decode_plain_text_with_accent() {
    let input = "Hello ÄäéÉñöåä";
    let result = decode_latex(input);
    assert_eq!(result, input)
}

#[test]
fn decode_plain_text_with_brace() {
    let result = decode_latex("Hello {world}");
    assert_eq!(result, "Hello world");
}

#[test]
fn decode_plain_text_with_escaped_brace() {
    let result = decode_latex(r"Hello \{world\}");
    assert_eq!(result, "Hello {world}");
}

#[test]
fn decode_plain_text_with_percentage() {
    let result = decode_latex("Hello %world%");
    assert_eq!(result, "Hello %world%");
}

#[test]
fn decode_plain_text_with_escaped_percentage() {
    let result = decode_latex(r"Hello \%world\%");
    assert_eq!(result, "Hello %world%");
}

#[test]
fn decode_plain_text_with_escaped_backslash() {
    let result = decode_latex(r"Hello \\world\\");
    assert_eq!(result, r"Hello \world\");
}

#[test]
fn decode_plain_text_with_backslash() {
    let result = decode_latex(r"Hello \ world");
    assert_eq!(result, "Hello  world");
}

#[test]
fn decode_with_triple_backslash() {
    let result = decode_latex(r"Hello \\\ world");
    assert_eq!(result, r"Hello \ world");
}

#[test]
fn decode_with_quad_backslash() {
    let result = decode_latex(r"Hello \\\\ world");
    assert_eq!(result, r"Hello \\ world");
}

#[test]
fn decode_umlaut_a() {
    let result = decode_latex("Hello \\\"a");
    assert_eq!(result, "Hello ä");
}

#[test]
fn decode_umlaut_o() {
    let result = decode_latex("Hello \\\"o");
    assert_eq!(result, "Hello ö");
}

#[test]
fn decode_ring_over_a() {
    let result = decode_latex(r"Hello \ra");
    assert_eq!(result, "Hello å");
}

#[test]
fn decode_ring_over_a_alt() {
    let result = decode_latex(r"Hello \aa");
    assert_eq!(result, "Hello å");
}

#[test]
fn decode_grave_accent() {
    let result = decode_latex(r"Hello \`o");
    assert_eq!(result, "Hello ò");
}

#[test]
fn decode_acute_accent() {
    let result = decode_latex(r"Hello \'o");
    assert_eq!(result, "Hello ó");
}

#[test]
fn decode_circumflex() {
    let result = decode_latex(r"Hello \^o");
    assert_eq!(result, "Hello ô");
}

#[test]
fn decode_long_umlaut() {
    let result = decode_latex(r"Hello \Ho");
    assert_eq!(result, "Hello o\u{030B}".nfc().collect::<String>());
}

#[test]
fn decode_tilde() {
    let result = decode_latex(r"Hello \~o");
    assert_eq!(result, "Hello õ");
}

#[test]
fn decode_cedilla() {
    let result = decode_latex(r"Hello \cc");
    assert_eq!(result, "Hello c\u{0327}".nfc().collect::<String>());
}

#[test]
fn decode_ogonek() {
    let result = decode_latex(r"Hello \ka");
    assert_eq!(result, "Hello a\u{0328}".nfc().collect::<String>());
}

#[test]
fn decode_barred() {
    let result = decode_latex(r"Hello \l");
    assert_eq!(result, "Hello \u{0142}".nfc().collect::<String>());
    //TODO: find correct character
}
#[test]
fn decode_macron() {
    let result = decode_latex(r"Hello \=o");
    assert_eq!(result, "Hello o\u{0304}".nfc().collect::<String>());
}
#[test]
fn decode_overdot() {
    let result = decode_latex(r"Hello \.a");
    assert_eq!(result, "Hello a\u{0307}".nfc().collect::<String>());
}
#[test]
fn decode_underdot() {
    let result = decode_latex(r"Hello \da");
    assert_eq!(result, "Hello a\u{0323}".nfc().collect::<String>());
}
#[test]
fn decode_breve() {
    let result = decode_latex(r"Hello \uo");
    assert_eq!(result, "Hello o\u{0306}".nfc().collect::<String>());
}
#[test]
fn decode_caron() {
    let result = decode_latex(r"Hello \vs");
    assert_eq!(result, "Hello s\u{030C}".nfc().collect::<String>());
}
#[test]
#[ignore]
fn decode_tie() {
    let result = decode_latex(r"Hello \t");
    assert_eq!(result, "Hello ".nfc().collect::<String>());
    //TODO: find correct character
}
#[test]
fn decode_slashed() {
    let result = decode_latex(r"Hello \o");
    assert_eq!(result, "Hello \u{00F8}");
}
#[test]
fn decode_dotless_i() {
    let result = decode_latex(r"Hello \^{\i}");
    assert_eq!(result, "Hello î");
}
#[test]
fn decode_dotless_j() {
    let result = decode_latex(r"Hello \^{\j}");
    assert_eq!(result, "Hello ĵ");
}

#[test]
fn decode_multiple_diacritics() {
    let result = decode_latex(r"Hello \~n\ra world");
    assert_eq!(result, "Hello ñå world");
}

#[test]
fn decode_chars_after_diacritic() {
    let result = decode_latex(r"Hello \'o world");
    assert_eq!(result, "Hello ó world");
}

#[test]
fn decode_diacritic_in_braces() {
    let result = decode_latex(r"Hello \'{o}");
    assert_eq!(result, "Hello ó");
}

#[test]
fn decode_double_brace() {
    let result = decode_latex(r"Hello \'{{o}}");
    assert_eq!(result, "Hello ó");
}

#[test]
fn decode_complex_string() {
    let result = decode_latex(r"Hello \'{o in braces} world \{\~{n\`o}\} after");
    assert_eq!(result, "Hello ó in braces world {ñò} after");
}
