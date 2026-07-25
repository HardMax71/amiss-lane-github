# Limits

The budget refuses a request above its ceiling rather than truncating it.

The ceiling is 8192 bytes, set by `CEILING` in [`src/budget.rs`](../src/budget.rs#L2), and a request
at exactly the ceiling is accepted.
