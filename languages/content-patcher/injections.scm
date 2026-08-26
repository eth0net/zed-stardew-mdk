; Dialogue inside a patch that targets a dialogue asset.
;
; The `Target` that says what kind of data this is sits beside `Entries` rather
; than above the strings, so one pattern has to match both pairs of the same
; object. Without that condition the injection would fire on file paths and
; event scripts too.
(object
  (pair
    key: (string (string_content) @_key)
    value: (string (string_content) @_target)
    (#eq? @_key "Target")
    (#match? @_target "Dialogue"))
  (pair
    key: (string (string_content) @_field)
    value: (object
      (pair value: (string (string_content) @injection.content)))
    (#any-of? @_field "Entries" "Fields"))
  (#set! injection.language "Stardew Dialogue"))
