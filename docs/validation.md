# Validation

`vscode-json-language-server`, driven by the schemas SMAPI publishes. The
extension downloads the server on first use and stages the vendored schemas into
its own work directory, so validation works offline.

`scripts/update-schemas.sh` refreshes the copies in `schemas/`. Review the diff
before committing: these are validation rules, and upstream tightens them
between game versions, so a refresh can start reporting errors in files that
were previously clean.

## What it can't see

Whether `{{Seasn}}` is a real token, whether `|valeuAt=` is a real filter,
whether a `FromFile` path exists, whether an `EditData` key matches the game's
data model. Content Patcher's `patch summary` console command remains the tool
for those.

One known false positive falls out of the same blindness: a `FromFile` whose
value is *entirely* a token is reported as missing its file extension, because
the schema matches the literal string and cannot expand `{{PathTexture}}` to see
the `.png`. Spelling the path out — `assets/paths_{{Season}}.png` — keeps the
tokens and loses the warning.

## When `content.json` isn't Content Patcher

Several frameworks use that filename. The extension reads the mod's
`manifest.json` and only applies the Content Patcher schema when the manifest
names `Pathoschild.ContentPatcher`. With no manifest at the worktree root — a
`Mods` folder opened whole — it assumes Content Patcher.

To point a file somewhere else, replace the association. Anything under
`settings` is merged over the extension's own configuration, so this wins:

```json
{
  "lsp": {
    "stardew-json-language-server": {
      "settings": {
        "json": {
          "schemas": [
            { "fileMatch": ["**/content.json"], "url": "./schemas/framework.json" }
          ]
        }
      }
    }
  }
}
```

A `url` starting with `./` or `~/` is resolved against the worktree, matching
Zed's own JSON support, so a pack that ships its own schema commits a
project-relative path rather than one that works on a single machine. Note that
`json.schemas` replaces the extension's list rather than adding to it, so restate
any association you still want.

A `"$schema"` key in the file itself also takes precedence over any association.

## Using your own language server build

```json
{
  "lsp": {
    "stardew-json-language-server": {
      "binary": { "path": "/path/to/vscode-json-language-server" }
    }
  }
}
```
