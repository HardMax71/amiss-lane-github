# amiss-lane-github

A test project for the [Amiss](https://github.com/HardMax71/amiss) GitHub provider lane. Its only
job is to be evaluated by a live controller against a real GitHub App, a real webhook, and a real
ruleset, so the lane's evidence comes from GitHub rather than from a local fixture.

The scenario is deliberate. [The limits note](docs/limits.md) describes what
[`src/budget.rs`](src/budget.rs) enforces. A pull request that changes one without the other is
documentation drift, and the lane must refuse it.
