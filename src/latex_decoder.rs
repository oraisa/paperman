use unicode_normalization::UnicodeNormalization;

pub fn decode_latex(latex: &str) -> String {
    let mut result = String::new();
    let mut iter = latex.chars();
    loop {
        let c_opt = iter.next();
        match c_opt {
            Some(c) => match c {
                '{' => (),
                '}' => (),
                '\\' => match iter.next() {
                    Some(c_next) => match c_next {
                        '"' => push_diacritic('\u{0308}', &mut iter, &mut result),
                        '\'' => push_diacritic('\u{0301}', &mut iter, &mut result),
                        '`' => push_diacritic('\u{0300}', &mut iter, &mut result),
                        '^' => push_diacritic('\u{0302}', &mut iter, &mut result),
                        'H' => push_diacritic('\u{030B}', &mut iter, &mut result),
                        '~' => push_diacritic('\u{0303}', &mut iter, &mut result),
                        'c' => push_diacritic('\u{0327}', &mut iter, &mut result),
                        'k' => push_diacritic('\u{0328}', &mut iter, &mut result),
                        // 'l' => push_diacritic('\u{0308}', &mut iter, &mut result),
                        '=' => push_diacritic('\u{0304}', &mut iter, &mut result),
                        'b' => push_diacritic('\u{0332}', &mut iter, &mut result),
                        '.' => push_diacritic('\u{0307}', &mut iter, &mut result),
                        'd' => push_diacritic('\u{0323}', &mut iter, &mut result),
                        'r' => push_diacritic('\u{030A}', &mut iter, &mut result),
                        'u' => push_diacritic('\u{0306}', &mut iter, &mut result),
                        'v' => push_diacritic('\u{030C}', &mut iter, &mut result),
                        // 't' => push_diacritic('\u{0308}', &mut iter, &mut result),
                        // 'o' => push_diacritic('\u{0308}', &mut iter, &mut result),
                        'a' => (),//push_diacritic('\u{0308}', &mut iter, &mut result),
                        // TODO: support for \i and \j
                        _ => result.push(c_next),
                    },
                    None => break
                }
                _ => result.push(c)
            },
            None => break
        }
    }
    return result.nfc().collect::<String>();
}

fn push_diacritic(combiner: char, iter: &mut std::str::Chars, result: &mut String) {
    match iter.next() {
        Some(c) => match c {
            '{' => match iter.next() {
                Some(c) => {
                    result.push(c);
                    result.push(combiner);
                },
                None => ()
            },
            _=> {
                result.push(c);
                result.push(combiner);
            },
        },
        None => ()
    }
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
#[ignore]
fn decode_long_umlaut() {
    let result = decode_latex(r"Hello \Ho");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}

#[test]
fn decode_tilde() {
    let result = decode_latex(r"Hello \~o");
    assert_eq!(result, "Hello õ");
}

#[test]
#[ignore]
fn decode_cedilla() {
    let result = decode_latex(r"Hello \cc");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}

#[test]
#[ignore]
fn decode_ogonek() {
    let result = decode_latex(r"Hello \ka");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}

#[test]
#[ignore]
fn decode_barred() {
    let result = decode_latex(r"Hello \l");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_macron() {
    let result = decode_latex(r"Hello \=o");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_overdot() {
    let result = decode_latex(r"Hello \-");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_underdot() {
    let result = decode_latex(r"Hello \d");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_breve() {
    let result = decode_latex(r"Hello \u");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_caron() {
    let result = decode_latex(r"Hello \v");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_tie() {
    let result = decode_latex(r"Hello \t");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_slashed() {
    let result = decode_latex(r"Hello \o");
    assert_eq!(result, "Hello ");
    //TODO: find correct character
}
#[test]
#[ignore]
fn decode_dotless_i() {
    let result = decode_latex(r"Hello \^{\i}");
    assert_eq!(result, "Hello î");
}
#[test]
#[ignore]
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
fn decode_complex_string() {
    let result = decode_latex(r"Hello \'{o in braces} world \{\~{n\`o}\} after");
    assert_eq!(result, "Hello ó in braces world {ñò} after");
}
