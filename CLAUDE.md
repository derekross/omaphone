# omaphone

Omarchy Mobile's Phone app. It uses omatel's shared crates
(https://github.com/derekross/omatel) as git dependencies pinned to a
release tag, and talks to the modem only through omatel-agent's
session-bus API (`omatel-api`).

Run `scripts/check.sh` before every commit: US spelling, rustfmt, clippy
with warnings as errors, and the tests. A clippy warning is a failure.

- Building, cross-compiling and testing: `docs/development.md`.
- **US English** in strings, identifiers, comments and docs
  (`scripts/spelling.sh`).
- **Privacy.** Never log a phone number, a name or message text: log
  states and counts only. Tests and screenshots use fictional numbers
  (`+1 202 555 01xx`) and obviously made-up names only.
- **Never** open a window or run a compositor on the developer's desktop:
  screenshots run in a headless Weston in a container
  (`scripts/ui-shots.sh`).
- **Emergency calls are not supported yet.** Phone shows a warning and never dials an emergency number. Never place or test an emergency call.
- A change in omatel's crates needs a new omatel tag first, then the tag
  bumped in `Cargo.toml` and `cargo update -p omatel-api` (one update
  moves every omatel crate).
