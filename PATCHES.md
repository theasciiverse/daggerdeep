# Changes from upstream

This is the Asciiverse packaging of [Dagger Deep](https://github.com/chadlung/daggerdeep)
by Chad Lung (GPL), built from upstream's v1.1.0. Upstream is the
`upstream` remote; the `asciiverse` branch carries the changes below.

## Code

- **Session logging.** `RUST_LOG=debug daggerdeep 2>daggerdeep.log`
  records every input event as it arrives, the menu choice, state and
  level changes, and a snapshot of the game after each turn (positions,
  hit points, the monsters left); at most twice a second in a hosted
  game. New file `src/logging.rs`; the input reads in `main.rs` and
  `menu.rs` go through it. Uses `log` and `env_logger`; nothing is
  printed unless `RUST_LOG` is set.

## Licence

- **COPYING**: the text of the GNU GPL version 3. Upstream's README names
  the GPL without a version, which under the GPL's own terms lets anyone
  use any version the FSF has published; the archives and system packages
  carry this copy of the licence with the binary. Worth offering upstream
  as a LICENSE file.

## Packaging (new files)

- `.github/workflows/release.yml`: av archives for Linux (static), macOS
  and Windows, each x86-64 and ARM64, plus SHA256SUMS, and .deb, .rpm,
  .apk and Arch packages. Replaces upstream's `build.yml`, which would
  also have run on our version tags and made a second release.
- `asciiverse/nfpm.yaml`: the system packages.
- `asciiverse/record-preview.sh`: records the catalog preview
  (`asciiverse/daggerdeep.cast`): the menu, then a fight in the first room.

## Ours

- **How to play it, at the top of README.md.** A short note on installing
  and starting it with av; the original text follows unchanged.
