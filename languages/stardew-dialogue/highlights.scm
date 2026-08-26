; Where two patterns capture the same node, Zed keeps the later one, so these
; run general to specific.

; Any `$` code. The two below narrow it.
(command) @function

; Portraits and emotions, including the numbered ones.
((command) @constant
  (#match? @constant "^\\$([0-9]+|[ahlsu])$"))

; Flow control — page breaks, questions and responses, prerequisites.
((command) @keyword
  (#match? @keyword "^\\$(b|c|d|e|k|p|q|r|t|v|y)$"))

(substitution) @variable.special

; `%fork` and `%revealtaste` read as substitutions but behave as commands.
((substitution) @keyword
  (#any-of? @keyword "%fork" "%revealtaste"))

(player) @variable.special
(variant_separator) @operator
(separator) @punctuation.delimiter

; `text` is deliberately not captured. The JSON layer underneath highlights
; Content Patcher tokens inside these strings, and capturing text here would
; paint over them.
