; Element names (in start tags, end tags, and self-closing tags)
(STag (Name) @tag)
(ETag (Name) @tag)
(EmptyElemTag (Name) @tag)

; Attribute names and values
(Attribute (Name) @attribute)
(AttValue) @string

; Comments
(Comment) @comment

; Processing instructions
(PI (PITarget) @keyword)

; CDATA sections
(CDSect) @string.special

; Entity and character references
(EntityRef) @constant
(CharRef) @constant

; XML declaration
(XMLDecl) @keyword

; Text content
(CharData) @string

; Punctuation
"<" @punctuation.bracket
">" @punctuation.bracket
"</" @punctuation.bracket
"/>" @punctuation.bracket
"<?" @punctuation.bracket
"?>" @punctuation.bracket
"=" @operator
