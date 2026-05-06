# Raster Dev Release Guard Walkthrough

This note is the quickest way to read the extra review model in `raster-dev-release-guard`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 106 | watch |
| stress | diagnostic quality | 177 | ship |
| edge | review cost | 134 | watch |
| recovery | safe rewrite | 152 | ship |
| stale | change width | 201 | ship |

Start with `stale` and `baseline`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The useful comparison is `change width` against `change width`, not the raw score alone.
