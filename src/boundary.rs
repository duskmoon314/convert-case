pub enum Boundary {
    Hyphen,
    Underscore,
    Space,

    UpperLower,
    LowerUpper,
    DigitUpper,
    UpperDigit,
    DigitLower,
    LowerDigit,
    //TwoChar(Box<dyn Fn(char, char) -> bool>),

    //UpperUpperLower, // Acronyms
    Acronyms,
    //ThreeChar(Box<dyn Fn(char, char, char) -> bool>), // more complex, should include index
}

impl Boundary {
    fn detect_one(&self, c: char) -> bool {
        use Boundary::*;
        match self {
            Hyphen => c == '-',
            Underscore => c == '_',
            Space => c == ' ',
            _ => false,
        }
    }

    fn detect_two(&self, c: char, d: char) -> bool {
        use Boundary::*;
        match self {
            UpperLower => c.is_uppercase() && d.is_lowercase(),
            LowerUpper => c.is_lowercase() && d.is_uppercase(),
            DigitUpper => c.is_ascii_digit() && d.is_uppercase(),
            UpperDigit => c.is_uppercase() && d.is_ascii_digit(),
            DigitLower => c.is_ascii_digit() && d.is_lowercase(),
            LowerDigit => c.is_lowercase() && d.is_ascii_digit(),
            _ => false,
        }
    }

    fn detect_three(&self, c: char, d: char, e: char) -> bool {
        use Boundary::*;
        match self {
            Acronyms => c.is_uppercase() && d.is_uppercase() && e.is_lowercase(),
            _ => false,
        }
    }
}

// gross
pub fn split(s: &str, boundaries: Vec<Boundary>) -> Vec<String> {

    let single_splits = s.chars().enumerate()
        .filter(|(_, c)| boundaries.iter().any(|b| b.detect_one(*c)))
        .map(|(i, _)| i + 1)
        .collect();

    let words = replace_at_indicies(s, single_splits);

    let final_words = words.iter().flat_map(|&w| {
        let left_iter = w.chars();
        let mid_iter = w.chars().skip(1);
        let right_iter = w.chars().skip(2);

        let three_iter = left_iter.clone()
            .zip(mid_iter.clone())
            .zip(right_iter);
        let two_iter = left_iter.clone().zip(mid_iter);

        let mut splits: Vec<usize> = three_iter.enumerate()
            .filter(|(_, ((c,d),e))| boundaries.iter().any(|b| b.detect_three(*c, *d, *e)))
            .map(|(i, _)| i + 1)
            .chain(
                two_iter.enumerate()
                        .filter(|(_, (c, d))| boundaries.iter().any(|b| b.detect_two(*c, *d)))
                        .map(|(i, _)| i + 1)
            )
            .collect();
        splits.sort();

        split_on_indicies(w, splits)
    });

    final_words.rev().map(ToString::to_string).filter(|s| !s.is_empty()).collect()
}

pub fn replace_at_indicies(s: &str, splits: Vec<usize>) -> Vec<&str> {
    let mut words = Vec::new();

    let mut first = s;
    let mut second;
    for &x in splits.iter().rev() {
        let pair = first.split_at(x);
        first = &pair.0[..(pair.0.len()-1)];
        second = pair.1;
        words.push(second);
    }
    words.push(first);

    words
}

pub fn split_on_indicies(s: &str, splits: Vec<usize>) -> Vec<&str> {
    let mut words = Vec::new();

    let mut first = s;
    let mut second;
    for &x in splits.iter().rev() {
        let pair = first.split_at(x);
        first = pair.0;
        second = pair.1;
        words.push(second);
    }
    words.push(first);
    
    words
}

// A boundary is either a replacement or not, maybe its Option<(usize, usize)>, where each
// index is what part to extract to make the word boundary.  If there is no replacement then
// its both are the same

