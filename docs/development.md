# Developing omaphone

## Build

```
bin/build        # cargo build --release --locked --bin omaphone
bin/install      # build and install the package (pkgbuild/PKGBUILD)
```

You need Rust, pkgconf, gtk4 (4.14 or later), libadwaita (1.5 or later)
and sqlite with their headers. omatel's crates come from git, at the tag
named in `Cargo.toml`, pinned in `Cargo.lock`.

## Cross-compile for the phone

`bin/cross` is omacam's (and omatel's) cross-compiler setup: the
`omacam-cross:1` Docker image and omacam's aarch64 sysroot when there is
one.

```
bin/cross sysroot     # only without omacam's sysroot
bin/cross build       # target/aarch64-unknown-linux-gnu/release/omaphone
bin/cross package     # pkgbuild/omaphone-<version>-1-aarch64.pkg.tar.zst
```

## Tests

- `scripts/check.sh`: US spelling, rustfmt, clippy `-D warnings`, the unit
  tests (the tel: link parser, time and duration text, call status and outcome icons, the script parser).
- `scripts/ui-shots.sh`: every page at 456x900 in a headless Weston inside
  a container (never on your desktop), on fictional demo data
  (`examples/demo-data.rs`), without the agent, into
  `target/ui-shots/omaphone/`.
- The agent, its D-Bus API and the whole chain down to the modem are
  tested in omatel. Its `scripts/ui-shots-live.sh` runs both apps, from
  checkouts next to omatel, with the agent and a scripted ModemManager.

The app takes a `--script` for screenshots; the steps are listed at the
top of `src/script.rs`.
