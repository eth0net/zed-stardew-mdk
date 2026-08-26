; Top-level sections, and each patch/token/config entry that nests further.
(pair
  key: (string (string_content) @name)
  value: [
    (object)
    (array)
  ]) @item

; Individual patches and dynamic tokens, named by what they act on. `LogName`
; is deliberately not matched: it would double up with `Target` on every patch
; that sets both.
(pair
  key: (string (string_content) @context)
  value: (string (string_content) @name)
  (#any-of? @context "Target" "Name")) @item
