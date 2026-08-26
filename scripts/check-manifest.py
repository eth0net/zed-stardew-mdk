#!/usr/bin/env python3
"""Consistency checks between extension.toml and languages/.

Covers the mistakes Zed reports quietly, or not at all:

- A grammar key that isn't snake_case, or doesn't match the parser's
  `tree_sitter_<key>` export, fails to link and leaves the language unhighlighted.
- A `not_in` bracket scope with no matching capture in overrides.scm makes Zed
  refuse to load the language outright.
- A language server naming a language that doesn't exist never starts.
"""

import pathlib
import re
import sys
import tomllib

root = pathlib.Path(__file__).resolve().parent.parent
manifest = tomllib.loads((root / "extension.toml").read_text())
problems = []

grammars = manifest.get("grammars", {})
if not grammars:
    problems.append("extension.toml declares no grammars")

for key in grammars:
    if not re.fullmatch(r"[a-z][a-z0-9_]*", key):
        problems.append(f"grammar key {key!r} is not snake_case; Zed requires it")

language_names = {}

for entry in manifest.get("languages", []):
    directory = root / entry
    config_path = directory / "config.toml"
    if not config_path.is_file():
        problems.append(f"{entry} is listed in extension.toml but has no config.toml")
        continue

    config = tomllib.loads(config_path.read_text())
    language_names[config.get("name", entry)] = entry

    grammar = config.get("grammar")
    if grammar is None:
        problems.append(f"{entry}/config.toml sets no grammar")
    elif grammar not in grammars:
        problems.append(
            f"{entry}/config.toml uses grammar {grammar!r}, "
            "which extension.toml does not declare"
        )

    overrides_path = directory / "overrides.scm"
    overrides = overrides_path.read_text() if overrides_path.is_file() else ""
    captured = set(re.findall(r"@([A-Za-z_][\w.]*)", overrides))
    for bracket in config.get("brackets", []):
        for scope in bracket.get("not_in", []):
            if scope not in captured:
                problems.append(
                    f'{entry}/config.toml has not_in = ["{scope}"] but '
                    f"overrides.scm does not capture @{scope}"
                )

for name, server in manifest.get("language_servers", {}).items():
    declared = server.get("languages", [])
    if language := server.get("language"):
        declared = [*declared, language]
    for language in declared:
        if language not in language_names:
            problems.append(
                f"language server {name!r} serves {language!r}, "
                "which this extension does not define"
            )
    for language in server.get("language_ids", {}):
        if language not in language_names:
            problems.append(
                f"language server {name!r} maps a language id for {language!r}, "
                "which this extension does not define"
            )

for problem in problems:
    print(f"error: {problem}", file=sys.stderr)

if problems:
    sys.exit(1)

print(f"extension.toml agrees with {len(language_names)} languages")
