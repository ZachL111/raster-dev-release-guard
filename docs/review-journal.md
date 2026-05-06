# Review Journal

The repository goal stays the same: build a Rust toolkit that studies release behavior through layout fixtures, with stable geometry snapshots and offline replay mode. This note explains the added review angle.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 106, lane `watch`
- `stress`: `diagnostic quality`, score 177, lane `ship`
- `edge`: `review cost`, score 134, lane `watch`
- `recovery`: `safe rewrite`, score 152, lane `ship`
- `stale`: `change width`, score 201, lane `ship`

## Note

A future change should add new cases before it changes the scoring rule.
