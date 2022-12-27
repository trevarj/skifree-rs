[![Build](https://github.com/trevarj/skifree-rs/actions/workflows/build.yml/badge.svg)](https://github.com/trevarj/skifree-rs/actions/workflows/build.yml)
# ⛷️ skifree-rs

A SkiFree clone written in Rust using the ggez game engine

![Start of the game v1.0.0](/screenshots/screenshot1.png)

## How to Play

```bash
git checkout https://github.com/trevarj/skifree-rs.git && cd skifree-rs
cargo install --path . # or just `cargo run --release`
```

### Download Binary from Releases

You can download a Linux x86_64 binary on the [Release page](https://github.com/trevarj/skifree-rs/releases)

## Controls

Key | Description
--- |---
Left/Right arrows | Move
Z or X            | Tricks 1 & 2
C                 | Flip

## Lots of TODOs

- [ ] AI for NPCs (noob, snowboarder, abominable snowman)
- [x] Wrap the map around when at edge
- [ ] Add slalom course
- [x] Add trick animations / controls
- [ ] Add scoring system
- [ ] Add game title at start
