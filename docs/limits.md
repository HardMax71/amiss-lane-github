# Limits

The budget refuses a request above its ceiling rather than truncating it.

The ceiling is 4096 bytes, set by `CEILING` in [`src/budget.rs`](src/budget.rs#L3), and a request
at exactly the ceiling is accepted.
