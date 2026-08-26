; Ordered specific to general: the first pattern to match a node wins.

; Flow control — page breaks, questions and responses, prerequisites.
((command) @keyword
  (#match? @keyword "^\\$(b|c|d|e|k|p|q|r|t|v|y)$"))

; Portraits and emotions, including the numbered ones.
((command) @constant
  (#match? @constant "^\\$([0-9]+|[ahlsu])$"))

; Whatever a future game version adds.
(command) @function

; `%fork` and `%revealtaste` read as substitutions but behave as commands.
((substitution) @keyword
  (#any-of? @keyword "%fork" "%revealtaste"))

(substitution) @variable.special

(player) @variable.special
(variant_separator) @operator
(separator) @punctuation.delimiter

; `text` is deliberately not captured. The JSON layer underneath highlights
; Content Patcher tokens inside these strings, and capturing text here would
; paint over them.
