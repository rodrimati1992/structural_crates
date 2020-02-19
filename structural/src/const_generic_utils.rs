// Helpers for operating on a `&str` const parameter,
// when `TStr_` internally uses a `&'static str` const parameter instead of
// a tuple of char type-level characters(a bunch of structs).

/// For converting a `tstr!(99)` to a 99 constant,for example.
pub(crate) const fn str_to_usize(s: &str) -> usize {
    const fn inner(s: &[u8], index: usize, curr: usize) -> usize {
        if index < s.len() {
            let digit = (s[index] - b'0') as usize;
            [(); 10][9 - digit];
            inner(s, index + 1, (curr * 10) + digit)
        } else {
            curr
        }
    }
    inner(s.as_bytes(), 0, 0)
}
