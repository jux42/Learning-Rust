pub fn slice_exo(s: &str) -> &str {
    for (i, &element) in s.as_bytes().iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}