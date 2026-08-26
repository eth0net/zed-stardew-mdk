# zed-stardew-mdk

[![CI](https://github.com/eth0net/zed-stardew-mdk/actions/workflows/ci.yaml/badge.svg)](https://github.com/eth0net/zed-stardew-mdk/actions/workflows/ci.yaml)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licence)
[![prek](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/j178/prek/master/docs/assets/badge-v0.json)](https://github.com/j178/prek)

A [Zed](https://zed.dev) extension for writing Stardew Valley mods. Highlights
[Content Patcher](https://github.com/Pathoschild/StardewMods/tree/develop/ContentPatcher)
tokens, dialogue and event scripts as the languages they are, and validates your
files against the schemas SMAPI publishes.

| File | Language | Validated against |
| --- | --- | --- |
| `content.json` | Content Patcher | [`content-patcher.json`](https://smapi.io/schemas/content-patcher.json) |
| `manifest.json` | Stardew JSON | [`manifest.json`](https://smapi.io/schemas/manifest.json) |
| `i18n/*.json` | Stardew JSON | [`i18n.json`](https://smapi.io/schemas/i18n.json) |
| the game's dialogue files | Stardew Dialogue Data | — |

> [!NOTE]
> **New, and not yet in the extensions store.** Install it as a dev extension
> for now — see [CONTRIBUTING.md](CONTRIBUTING.md).

## Install

Once published: install **Stardew Valley MDK** from Zed's extensions store. The
language server is downloaded on first use.

Zed matches languages on filenames rather than paths, so anything named per-pack
needs a glob. In the project's `.zed/settings.json`:

```json
{
  "file_types": {
    "Stardew JSON": ["**/i18n/*.json"],
    "Content Patcher": ["**/includes/*.json"],
    "Stardew Dialogue Data": ["**/Dialogue/*/Dialogue.json"]
  }
}
```

The second line is for files pulled in by an `Include` patch, which can be named
anything.

## Highlighting

A `speak` line in an event is JSON, then event script, then dialogue, and each
layer is highlighted as itself — `$` portrait codes and `#` page breaks inside a
quoted argument inside a `/`-delimited command inside a JSON string. Content
Patcher tokens keep their own colour wherever they appear.

Syntax comes from three grammars of its own rather than Zed's JSON one, because
SMAPI's dialect allows comments, trailing commas and single quotes, and because
tokens are worth parsing rather than pattern-matching.
[docs/highlighting.md](docs/highlighting.md) has the detail.

## Validation

Diagnostics, completions and hover documentation come from
`vscode-json-language-server` driven by SMAPI's own schemas — the `Action` and
`PatchMode` enums, required fields per patch, the `Target` pattern rejecting
`Content/` prefixes and `.xnb` extensions. The schemas are vendored, so this
works offline and doesn't shift under you mid-session.

It cannot check what a JSON schema can't see: whether `{{Seasn}}` is a real
token, whether `|valeuAt=` is a real filter, or whether a `FromFile` path
exists. Content Patcher's `patch summary` remains the tool for that.
[docs/validation.md](docs/validation.md) covers the schema associations, the one
known false positive, and how to point a file at your own schema.

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md). The short version: the grammars live in
their own repositories and are pinned by commit, so a grammar change is pushed
there and picked up with `scripts/bump-grammar-rev.py`.

## Credits

- [Pathoschild/SMAPI](https://github.com/Pathoschild/SMAPI) publishes the
  schemas at [smapi.io/schemas](https://smapi.io/schemas/), which do all the
  validation work here.
- [linkoid/stardew-syntax](https://github.com/linkoid/stardew-syntax) — prior
  art for editor support, and where the idea of treating Content Patcher tokens
  as an embedded language came from. Its grammars are TextMate and GPL-3.0; no
  code was taken from it, and the syntax modelled here comes from Content
  Patcher's own author guide.
- [bsavage81/stardew-modding-schema](https://github.com/bsavage81/stardew-modding-schema)
  — the same idea for VS Code, and a source of schemas for frameworks beyond
  Content Patcher.

## Licence

`MIT OR Apache-2.0`, at your option — see [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).

`schemas/` holds verbatim copies of SMAPI's schemas, which are LGPL-3.0 like the
rest of SMAPI. They keep that licence; see [licenses/smapi](licenses/smapi).

Developed with [Claude Code](https://claude.com/claude-code).
