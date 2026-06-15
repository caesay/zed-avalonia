; Fold elements that have content (start tag + content + end tag)
(element
  (STag) @fold.start
  (ETag) @fold.end)

; Fold comments
(Comment) @fold
