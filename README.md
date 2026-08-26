# zed-stardew-mdk

[![CI](https://github.com/eth0net/zed-stardew-mdk/actions/workflows/ci.yml/badge.svg)](https://github.com/eth0net/zed-stardew-mdk/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licence)
[![prek](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/j178/prek/master/docs/assets/badge-v0.json)](https://github.com/j178/prek)

A [Zed](https://zed.dev) extension for writing Stardew Valley mods. Highlights
[Content Patcher](https://github.com/Pathoschild/StardewMods/tree/develop/ContentPatcher)
tokens as syntax and validates your files against the schemas SMAPI publishes.

| File | Language | Validated against |
| --- | --- | --- |
| `content.json` | Content Patcher | [`content-patcher.json`](https://smapi.io/schemas/content-patcher.json) |
| `manifest.json` | Stardew JSON | [`manifest.json`](https://smapi.io/schemas/manifest.json) |
| `i18n/*.json` | Stardew JSON | [`i18n.json`](https://smapi.io/schemas/i18n.json) |
| the game's dialogue files | Stardew Dialogue Data | — |

> [!NOTE]
> **New, and not yet in the extensions store.** Install it as a dev extension
> for now — see [CONTRIBUTING.md](CONTRIBUTING.md). Diagnostics, completions and
> hover all work; the gaps are listed under [Validation](#validation).

## Install

Once published: install **Stardew Valley MDK** from Zed's extensions store.

The language server is downloaded on first use. To use your own build instead:

```json
{
  "lsp": {
    "stardew-json-language-server": {
      "binary": { "path": "/path/to/vscode-json-language-server" }
    }
  }
}
```

`i18n` files need one setting, because Zed matches languages on filenames rather
than paths. In the project's `.zed/settings.json`:

```json
{
  "file_types": {
    "Stardew JSON": ["**/i18n/*.json"],
    "Content Patcher": ["**/includes/*.json"],
    "Stardew Dialogue Data": ["**/Dialogue/*/Dialogue.json"]
  }
}
```

The second line is for files pulled in by an `Include` patch, which can be
named anything.

## Highlighting

Syntax comes from
[tree-sitter-stardew-json](https://github.com/eth0net/tree-sitter-stardew-json)
rather than Zed's JSON grammar, for two reasons.

**SMAPI's dialect is not strict JSON.** It parses with Json.NET, so `//` and
`/* */` comments, trailing commas and single-quoted strings are all legal — and
Content Patcher's own documentation uses the single quotes. A strict grammar
reports each of these as a syntax error.

**Content Patcher tokens are structured.** They are parsed rather than
pattern-matched, so nesting and filters come out as syntax:

```jsonc
{
  // A token taking input highlights as a call; a bare one as a variable.
  "FromFile": "assets/{{Random: sun, rain |key={{Day}}}}_{{Season}}.png"
}
```

That covers tokens in values and in keys, mod-provided names
(`{{Some.Mod/Token}}`), input arguments, `|filter=value` arguments and
arbitrary nesting.

### Dialogue

Dialogue is a language of its own inside those strings, and it gets highlighted
as one — `$` portrait and control codes, `%` substitutions, `@` for the player's
name, `^` splitting male from female text, and `#` page breaks:

```jsonc
{
  "Target": "Characters/Dialogue/Abigail",
  "Entries": {
    "Introduction": "Hi, I'm Abigail.$h#$b#Nice weather for {{Season}}, @."
  }
}
```

It fires only where the data really is dialogue. In a content pack that means a
patch whose `Target` names a dialogue asset — the `Target` sits beside `Entries`
rather than above it, so the query matches both, and file paths and event
scripts are left alone. The game's own dialogue files carry no such marker, so
they get their own language, reached through a glob.

Content Patcher tokens inside a dialogue string keep their own highlighting.

Event scripts — the `/`-delimited commands under `Data/Events` — are **not**
highlighted yet.

## Validation

Diagnostics, completions and hover documentation come from
`vscode-json-language-server` driven by SMAPI's own schemas: the `Action` and
`PatchMode` enums, required fields per patch, the `Target` pattern that rejects
`Content/` prefixes and `.xnb` extensions, and every field's description on
hover.

The schemas are vendored, so validation works offline and does not shift under
you mid-session. `scripts/update-schemas.sh` refreshes them.

What it does **not** check is anything a JSON schema cannot see: whether
`{{Seasn}}` is a real token, whether `|valeuAt=` is a real filter, whether a
`FromFile` path exists, or whether an `EditData` key matches the game's data
model. The same blindness cuts the other way: a `FromFile` that is *entirely* a
token is reported as missing its file extension, because the schema matches the
literal string and cannot expand `{{PathTexture}}` to see the `.png`. Content Patcher's `patch summary` console command remains the tool for
that. Closing that gap needs a language server that understands Content
Patcher, which is [tracked separately](https://github.com/eth0net/zed-stardew-mdk/issues).

### When `content.json` isn't Content Patcher

Several frameworks use that filename. The extension reads the mod's
`manifest.json` and only applies the Content Patcher schema when the manifest
names `Pathoschild.ContentPatcher`; with no manifest at the worktree root — a
`Mods` folder opened whole — it assumes Content Patcher.

To point a file somewhere else, replace the association. Anything under
`settings` is merged over the extension's own configuration, so this wins. A
`url` starting with `./` or `~/` is resolved against the worktree, matching
Zed's own JSON support, so a pack that ships its own schema can commit a
project-relative path rather than one that only works on one machine:

```json
{
  "lsp": {
    "stardew-json-language-server": {
      "settings": {
        "json": {
          "schemas": [
            { "fileMatch": ["**/content.json"], "url": "https://example.com/framework.schema.json" }
          ]
        }
      }
    }
  }
}
```

A `"$schema"` key in the file itself also takes precedence.

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md). The short version: the grammar lives in
its own repository and is pinned by commit, so grammar changes are pushed there
and then picked up with `scripts/bump-grammar-rev.py`.

## Credits

- [Pathoschild/SMAPI](https://github.com/Pathoschild/SMAPI) publishes the
  schemas at [smapi.io/schemas](https://smapi.io/schemas/), which do all the
  validation work here.
- [linkoid/stardew-syntax](https://github.com/linkoid/stardew-syntax) — prior
  art for editor support, and where I got the idea of treating Content Patcher
  tokens as an embedded language. Its grammars are TextMate and GPL-3.0; no code
  was taken from it, and the token syntax modelled here comes from Content
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
