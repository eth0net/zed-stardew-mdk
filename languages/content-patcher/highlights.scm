; Object keys read as properties, including keys built from tokens.
(pair key: (string) @property)

(string) @string
(escape_sequence) @string.escape

(number) @number
[
  (true)
  (false)
] @boolean
(null) @constant.builtin

(comment) @comment

; --- Content Patcher tokens -------------------------------------------------
; A token that takes input reads as a call (`{{Random: a, b}}`); a bare one
; reads as a variable (`{{Season}}`). Order matters: the first pattern wins.

(token
  name: (token_name) @function
  .
  ":")

(token name: (token_name) @variable.special)

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
