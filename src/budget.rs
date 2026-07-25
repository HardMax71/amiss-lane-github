/// A request above this many bytes is refused rather than truncated.
pub const CEILING: usize = 8192;

pub fn accepts(request: usize) -> bool {
    request <= CEILING
}
