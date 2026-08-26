#!/usr/bin/env python3
"""Point extension.toml at the grammar repository's current HEAD.

Zed fetches grammars over git and compiles the committed `src/parser.c`, so a
grammar change is only visible here once it has been pushed and this pin moved.
"""

import pathlib
import re
import subprocess
import sys
import tomllib

GRAMMAR = "stardew_json"

root = pathlib.Path(__file__).resolve().parent.parent
path = root / "extension.toml"
manifest = tomllib.loads(path.read_text())

entry = manifest.get("grammars", {}).get(GRAMMAR)
if entry is None:
    sys.exit(f"error: extension.toml has no [grammars.{GRAMMAR}]")

repository = entry["repository"]
ref = sys.argv[1] if len(sys.argv) > 1 else "HEAD"

remote = subprocess.run(
    ["git", "ls-remote", repository, ref],
    capture_output=True,
    text=True,
    check=True,
)
if not remote.stdout.strip():
    sys.exit(f"error: {repository} has no ref {ref!r}")

sha = remote.stdout.split()[0]
if sha == entry.get("rev"):
    print(f"already at {sha}")
    raise SystemExit(0)

# Rewrite in place rather than re-serialising: the manifest carries comments
# that a round-trip through tomllib would drop.
lines = path.read_text().splitlines(keepends=True)
section = False
for i, line in enumerate(lines):
    if line.startswith("["):
        section = line.strip() == f"[grammars.{GRAMMAR}]"
    elif section and (match := re.match(r"(rev|commit)( *= *).*\n?", line)):
        lines[i] = f"{match.group(1)}{match.group(2)}\"{sha}\"\n"
        break
else:
    sys.exit(f"error: [grammars.{GRAMMAR}] has no rev or commit to update")

path.write_text("".join(lines))
print(f"{entry.get('rev')} -> {sha}")
