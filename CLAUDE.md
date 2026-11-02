# Dagger Deep (Asciiverse packaging)

Chad Lung's terminal roguelike in Rust and Ratatui, solo or two-player
co-op over TCP; packaged for the Asciiverse catalog and as system
packages. Not our code: keep changes small, list every one in PATCHES.md,
and offer fixes upstream.

- Remotes: `upstream` is github.com/chadlung/daggerdeep, `origin` is
  github.com/theasciiverse/daggerdeep.
- Branch `asciiverse` = upstream main (v1.1.0) plus our commits.
- Release: tag `v<version>-av.<n>` and push it.
- Session logging: `RUST_LOG=debug daggerdeep 2>daggerdeep.log`.
- Needs 100x30 or more; refuses to start smaller.
- Co-op: the host listens on TCP 4444, the other player joins by IP.
- Preview: `asciiverse/record-preview.sh`, then copy the cast to
  `hub/site/casts/`.
