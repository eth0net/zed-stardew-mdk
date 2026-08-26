; Where two patterns capture the *same* node, Zed keeps the later one, so these
; run general to specific. Nesting is unaffected — an inner node sorts after its
; parent and wins on its own.

(string) @string
(escape_sequence) @string.escape

; Object keys, including keys built from tokens.
(pair key: (string) @property)

(number) @number
[
  (true)
  (false)
] @boolean
(null) @constant.builtin

(comment) @comment

; --- Content Patcher tokens -------------------------------------------------

(token
  [
    "{{"
    "}}"
  ] @punctuation.special)

(token ":" @punctuation.delimiter)
(token_input "," @punctuation.delimiter)
(token_input (token_text) @string.special)

(token_filter "|" @punctuation.special)
(token_filter name: (token_filter_name) @attribute)
(token_filter "=" @operator)

; A bare token reads as a variable; one that takes input reads as a call. The
; call pattern comes second so it wins where both match.
(token name: (token_name) @variable.special)
(token
  name: (token_name) @function
  .
  ":")

; --- structure -------------------------------------------------------------

(pair ":" @punctuation.delimiter)

(object
  [
    "{"
    "}"
  ] @punctuation.bracket)

(array
  [
    "["
    "]"
  ] @punctuation.bracket)

(object "," @punctuation.delimiter)
(array "," @punctuation.delimiter)
