; Where two patterns capture the same node, Zed keeps the later one, so these
; run general to specific.

(string) @string
(separator) @punctuation.delimiter

; Coordinates, frame numbers, delays.
((word) @number
  (#match? @number "^-?[0-9]+(\\.[0-9]+)?$"))

; The first word of a segment names the command. Excluding numbers keeps the
; positional opening segments — music, then viewport coordinates — from having
; their first number read as a command.
((segment . (word) @function)
  (#not-match? @function "^-?[0-9]+$"))

; Other words — actor names, locations, flags, event ids — are deliberately left
; uncaptured, so they keep the enclosing string's colour rather than turning the
; script into a rainbow.
