# Highlighting

Three grammars, layered. The JSON one parses the file; the other two are
injected into the strings that hold them.

- [tree-sitter-stardew-json](https://github.com/eth0net/tree-sitter-stardew-json)
- [tree-sitter-stardew-dialogue](https://github.com/eth0net/tree-sitter-stardew-dialogue)
- [tree-sitter-stardew-event](https://github.com/eth0net/tree-sitter-stardew-event)

## Why not Zed's JSON grammar

SMAPI parses with Json.NET, so `//` and `/* */` comments, trailing commas and
single-quoted strings are all legal — Content Patcher's own documentation uses
the single quotes. A strict grammar reports each as a syntax error.

Tokens are parsed rather than pattern-matched, so nesting and filters come out
as syntax:

```jsonc
{
  // A token taking input highlights as a call; a bare one as a variable.
  "FromFile": "assets/{{Random: sun, rain |key={{Day}}}}_{{Season}}.png"
}
```

That covers tokens in values and in keys, mod-provided names
(`{{Some.Mod/Token}}`), input arguments, `|filter=value` arguments and arbitrary
nesting.

## Dialogue

`$` portrait and control codes, `%` substitutions, `@` for the player's name,
`^` splitting male from female text, `#` page breaks:

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
rather than above the strings, so the query matches both pairs of the same
object, and file paths and event scripts are left alone. The game's own dialogue
files carry no such marker, so they get their own language reached through a
glob.

Dialogue prose keeps the string colour deliberately. Only the codes are
captured, so a Content Patcher token inside a dialogue string keeps its own
highlighting rather than being painted over.

## Event scripts

The `/`-delimited command lists under `Data/Events`, which nest one level
further — a `speak` line is JSON, then event script, then dialogue:

```jsonc
{
  "Target": "Data/Events/Town",
  "Entries": {
    "42/f Abigail 750/t 600 1200": "continue/64 15/skippable/speak Abigail \"Morning, @!$h\"/end"
  }
}
```

Command names come from the first word of each segment rather than a fixed list.
59 distinct commands turn up in a single content pack and the set changes between
game releases, so a new one needs no update here. Numbers are classified the same
way, by the query rather than the grammar, because a rule matching them would
split an identifier like `120451_MeetInTown` after its digits.

Arguments — actor names, locations, flags — are deliberately left uncaptured, so
they keep the enclosing string's colour instead of turning a script into a
rainbow.

## A note on themes

Where two patterns capture the same node, Zed keeps the later one, so these
queries run general to specific. Nesting is unaffected: an inner node sorts
after its parent and wins on its own.

Separators are `punctuation.delimiter` and token braces are
`punctuation.special`, which is correct but invisible in a theme that defines
neither — GitHub Dark Default is one. `theme_overrides` in your Zed settings
fixes that per theme.
