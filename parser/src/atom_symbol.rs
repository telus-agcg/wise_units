use crate::{tokens::AtomSymbol, ParseResult};

// x = Unitwise.search '.*'
// x.flat_map { |u| u.terms.map { |t| t.atom.primary_code } }.uniq.sort
// x.flat_map { |u| u.terms.map { |t| t.atom.primary_code } }.uniq.join('').chars.uniq.sort
//
//
const TEN_STAR: &str = "10*";
const TEN_CARET: &str = "10^";

pub(crate) fn parse<'input>(input: &'input str) -> ParseResult<'_, AtomSymbol<'input>> {
    // Check these special cases, since they don't adhere to normal parsing rules.
    if input.starts_with(TEN_STAR) {
        return Ok((AtomSymbol(TEN_STAR), &input[3..]));
    } else if input.starts_with(TEN_CARET) {
        return Ok((AtomSymbol(TEN_CARET), &input[3..]));
    }

    let mut chars = input.chars().peekable();
    let mut atom_len = 0_usize;
    let mut in_bracket = false;

    // Make sure the first char is something legal.
    match chars.next() {
        Some(next) if next == '[' => {
            in_bracket = true;
            atom_len += 1;
        }
        Some(next) if is_valid_char(next) => {
            atom_len += 1;
        }
        _ => return Err(input),
    }

    while let Some(next) = chars.peek() {
        if in_bracket {
            if next == &']' {
                in_bracket = false;
                atom_len += 1;
                break;
            } else if is_valid_bracketed_char(*next) {
                atom_len += 1;
                chars.next();
            } else {
                break;
            }
        } else if next == &'[' {
            in_bracket = true;
            atom_len += 1;
            chars.next();
        } else if is_valid_char(*next) {
            atom_len += 1;
            chars.next();
        } else {
            break;
        }
    }

    if in_bracket {
        return Err(input);
    }

    let atom_symbol: &str = &input[..atom_len];
    let next_index = atom_symbol.len();

    Ok((AtomSymbol(atom_symbol), &input[next_index..]))
}

///// UCUM spec 2.1.5.1.2:
/////
///// > Within a matching pair of square brackets the full range of characters
///// > 33–126 can be used.
/////
// fn parse_bracketed<'input>(input: &'input str) -> ParseResult<&'input str> {
//     let mut chars = input.chars().peekable();
// }

// Can't contain:
//
// * multiplication char: .
// * division char: /
// * numbers
// * curly braces
// * parentheses
//
// Used chars in UCUM spec:
//
// %, ', (, ), *, ., /,
//
// 0, 1, 2, 3, 5, 6, 9,
//
// A, B, C, D, E, F, G, H, I, J, K, L, M, N,
// O, P, Q, R, S, T, U, V, W, X,
//
// [, ], ^, _,
//
// a, b, c, d, e, f, g, h, i, j, k, l, m, n,
// o, p, q, r, s, t, u, v, w, x, y, z
//
// ...but these have special meanings, so don't allow in atoms:
//
// * parentheses
// * multiplier (.), divider (/)
// * brackets enclose what can be any term, so we handle those specially elsewhere.
//
///
/// UCUM Spec 2.1.3.3
///
/// A terminal unit symbol can not consist of only digits (‘0’–‘9’) because
/// those digit strings are interpreted as positive integer numbers. However, a
/// symbol “10*” is allowed because it ends with a non-digit allowed to be part
/// of a symbol.
///
///
/// ---
///
/// As such, this must make sure the Atom doesn't start with a number.
///
/// UCUM Spec 2.1.3.2
///
/// Terminal unit symbols can consist of all ASCII characters in the range of
/// 33–126 (0x21–0x7E) excluding double quotes (‘"’), parentheses (‘(’ and ‘)’),
/// plus sign (‘+’'), minus sign (‘-’'), period (‘.’'), solidus (‘/’'), equal
/// sign (‘=’'), square brackets (‘[’ and ‘]’), and curly braces (‘{’ and ‘}’),
/// which have special meaning.
///
fn is_valid_char(c: char) -> bool {
    match c {
        'A'..='Z' | 'a'..='z' | '%' | '\'' | '*' | '^' | '_' => true,
        _ => false,
    }
}

fn is_valid_bracketed_char(c: char) -> bool {
    match c {
        '(' | ')' | '.'..='9' | 'A'..='Z' | 'a'..='z' | '%' | '\'' | '*' | '^' | '_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::AtomSymbol;

    macro_rules! assert_parse {
        ($input:expr, $expected_token:expr, $expected_remaining:expr) => {
            assert_eq!(
                super::parse($input),
                Ok((AtomSymbol($expected_token), $expected_remaining)),
            );
        };
    }

    #[test]
    fn test_all_the_atoms_tail_shift1234() {
        for (primary_code, secondary_code) in crate::test_helper::ATOMS.iter() {
            let tail = "!@#$";
            let primary_input = format!("{}{}", primary_code, tail);
            assert_parse!(&primary_input, primary_code, tail);

            let secondary_input = format!("{}{}", secondary_code, tail);
            assert_parse!(&secondary_input, secondary_code, tail);
        }
    }

    #[test]
    fn test_all_the_atoms_tail_1234() {
        for (primary_code, secondary_code) in crate::test_helper::ATOMS.iter() {
            let tail = "1234";
            let primary_input = format!("{}{}", primary_code, tail);
            assert_parse!(&primary_input, primary_code, tail);

            let secondary_input = format!("{}{}", secondary_code, tail);
            assert_parse!(&secondary_input, secondary_code, tail);
        }
    }

    #[test]
    fn test_all_the_atoms_bracket_tail() {
        for (primary_code, secondary_code) in crate::test_helper::ATOMS.iter() {
            let tail = "]";
            let primary_input = format!("{}{}", primary_code, tail);
            assert_parse!(&primary_input, primary_code, tail);

            let secondary_input = format!("{}{}", secondary_code, tail);
            assert_parse!(&secondary_input, secondary_code, tail);
        }
    }

    #[test]
    fn test_parse_errors() {
        assert_eq!(super::parse("0"), Err("0"));
        assert_eq!(super::parse(""), Err(""));
        assert_eq!(super::parse("[asdf"), Err("[asdf"));
    }
}
