; A quoted argument in an event script is what the character says, so it is
; dialogue: `speak Abigail "Hey @!$h"` and `question fork0 "#Yes#No"` both carry
; the same codes as a dialogue file.
((string_content) @injection.content
  (#set! injection.language "Stardew Dialogue"))
