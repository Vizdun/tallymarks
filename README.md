Crate for converting numbers to tally marks.

# Usage

Without spaces:

```rust
assert_eq!("𝍸𝍸", tallymarks::tally_marks(10));
```

With spaces:

```rust
assert_eq!("𝍸 𝍸", tallymarks::tally_marks_spaced(10));
```