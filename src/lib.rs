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

const TALLY_MARKS: [&str; 6] = ["", "𝍩", "𝍪", "𝍫", "𝍬", "𝍸"];

pub fn tally_marks(n: usize) -> String {
    let full = n / 5;
    let rem = n % 5;

    let rem_str = TALLY_MARKS[rem];

    TALLY_MARKS[5].repeat(full) + rem_str
}

pub fn tally_marks_spaced(n: usize) -> String {
    tally_marks(n).chars().fold(String::new(), |acc, c| {
        if acc != "" {
            format!("{} {}", acc, c)
        } else {
            format!("{}{}", acc, c)
        }
    })
}
