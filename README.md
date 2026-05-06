# raster-dev-release-guard

`raster-dev-release-guard` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies release behavior through layout fixtures, with stable geometry snapshots and offline replay mode.

## Purpose

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how change width and review cost should influence a review result.

## Raster Dev Release Guard Review Notes

The first comparison I would make is `change width` against `change width` because it shows where the rule is most opinionated.

## What Is Covered

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/raster-dev-release-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Implementation Notes

The repository has two validation layers: the original compact policy fixture and the domain review fixture. They are separate so one can change without hiding failures in the other.

The Rust code keeps the review rule close to the tests.

## Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Audit Path

The verifier is intentionally local. It should fail if the fixture score math, lane assignment, or language-specific test drifts.

## Limits

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
