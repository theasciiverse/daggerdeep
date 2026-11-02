#!/bin/sh
# Records the catalog preview, asciiverse/daggerdeep.cast, at 100x30 with
# hub's castrec (hub/tools/castrec.py, beside this repo in the Asciiverse
# tree): the menu, then single player, circling in the first room so the
# monsters come to the player and a fight starts. Every dungeon is new, so
# every recording differs; check it shows a fight before keeping it.
set -e
cd "$(dirname "$0")/.."
cargo build --release
PATH="$PWD/target/release:$PATH" python3 -I ../../hub/tools/castrec.py \
  asciiverse/daggerdeep.cast . \
  "w2.5;ks;w1.5;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;r3:right:0.35;r3:down:0.35;r3:left:0.35;r3:up:0.35;w1.5;kesc;w1;ky;w0.5" \
  -- daggerdeep
