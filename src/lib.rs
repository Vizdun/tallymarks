#[cfg(test)]
mod tests {
    #[test]
    fn tally_marks() {
        assert_eq!("", crate::tally_marks(0));
        assert_eq!("𝍩", crate::tally_marks(1));
        assert_eq!("𝍪", crate::tally_marks(2));
        assert_eq!("𝍫", crate::tally_marks(3));
        assert_eq!("𝍬", crate::tally_marks(4));
        assert_eq!("𝍸", crate::tally_marks(5));
        assert_eq!("𝍸𝍸", crate::tally_marks(10));
        assert_eq!("𝍸𝍸𝍸𝍸𝍸𝍸𝍸𝍸𝍪", crate::tally_marks(42));
    }

    #[test]
    fn tally_marks_spaced() {
        assert_eq!("", crate::tally_marks_spaced(0));
        assert_eq!("𝍩", crate::tally_marks_spaced(1));
        assert_eq!("𝍪", crate::tally_marks_spaced(2));
        assert_eq!("𝍫", crate::tally_marks_spaced(3));
        assert_eq!("𝍬", crate::tally_marks_spaced(4));
        assert_eq!("𝍸", crate::tally_marks_spaced(5));
        assert_eq!("𝍸 𝍸", crate::tally_marks_spaced(10));
        assert_eq!("𝍸 𝍸 𝍸 𝍸 𝍸 𝍸 𝍸 𝍸 𝍪", crate::tally_marks_spaced(42));
    }
}

const TALLY_MARKS: [char; 5] = ['𝍩', '𝍪', '𝍫', '𝍬', '𝍸'];

fn gen_marks(n: usize) -> impl Iterator<Item = char> {
    let full = n / 5;
    let rem = n % 5;

    let rem_char = if rem != 0 {
        Some(TALLY_MARKS[rem - 1])
    } else {
        None
    };

    std::iter::repeat(TALLY_MARKS[4]).take(full).chain(rem_char)
}

pub fn tally_marks(n: usize) -> String {
    String::from_iter(gen_marks(n))
}

pub fn tally_marks_spaced(n: usize) -> String {
    let mut chars = gen_marks(n).peekable();
    let mut marks = String::with_capacity(chars.size_hint().0 * 4);
    while let Some(c) = chars.next() {
        marks.push(c);
        if chars.peek().is_some() {
            marks.push(' ')
        }
    }
    marks
}
