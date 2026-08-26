; The first word of a segment names the command. Excluding numbers keeps the
; positional opening segments — music, then viewport coordinates — from having
; their first number read as a command.
((segment . (word) @function)
  (#not-match? @function "^-?[0-9]+$"))

; Coordinates, frame numbers, delays.
((word) @number
  (#match? @number "^-?[0-9]+(\\.[0-9]+)?$"))

; Actor names, locations, flags, event ids.
(word) @variable

(string) @string
(separator) @punctuation.delimiter
