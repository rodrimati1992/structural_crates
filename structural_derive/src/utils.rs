use proc_macro2::Span;

pub(crate) trait SpanExt {
    fn combine_span(self, other: Span) -> Span;
}

impl SpanExt for Span {
    fn combine_span(self, other: Span) -> Span {
        self.join(other).unwrap_or(self)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn remove_raw_prefix(mut s: String) -> String {
    if s.starts_with("r#") {
        s.drain(..2);
    }
    s
}
