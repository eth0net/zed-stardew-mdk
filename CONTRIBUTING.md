# Contributing

## Getting set up

```sh
git clone https://github.com/eth0net/zed-stardew-mdk
cd zed-stardew-mdk
cargo build --release --target wasm32-wasip2
```

`rust-toolchain.toml` declares the wasm target, so there is nothing to add by
hand.

To run it: **zed: extensions** → **Install Dev Extension** → this directory.
Zed builds the extension, fetches the grammar and compiles it. After a change,
**zed: reload extensions**.

`examples/ExampleContentPack` is a content pack that exercises most of the
grammar and both schemas. Opening `content.json` from it is the quickest way to
see whether a change worked.

## The grammar lives elsewhere

Syntax comes from
[eth0net/tree-sitter-stardew-json](https://github.com/eth0net/tree-sitter-stardew-json),
pinned by commit in `extension.toml`. Zed fetches grammars over git, so a
grammar change has to be pushed there before this extension can use it:

```sh
# in the grammar repo
scripts/check.sh && git commit && git push

# here
scripts/bump-grammar-rev.py
```

Highlight and outline queries live *here*, in `languages/`, because they are
Zed-specific. A query referring to a node the grammar doesn't have is silently
dropped, so check the log — Zed reports unrecognised captures.

## Before opening a pull request

```sh
cargo fmt --all
cargo clippy --all-targets    # must be warning-free
cargo build --release --target wasm32-wasip2
```

Or let the hooks run them:

```sh
brew install prek
prek install
```

Committing runs the file-hygiene checks and `cargo fmt`; pushing adds `clippy`
and the wasm build, which is everything CI runs.

## Schemas

`schemas/` holds copies of the schemas SMAPI publishes.
`scripts/update-schemas.sh` refreshes them. Review the diff before committing:
these are validation rules, and upstream tightens them between game versions,
so a refresh can start reporting errors in files that were previously clean.

## Attribution

Developed with [Claude Code](https://claude.com/claude-code).

## Licensing

Contributions are dual licensed under MIT and Apache-2.0, matching the project.
By submitting a pull request you agree to that.
