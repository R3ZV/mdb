package token

type Type int

const (
	H1 Type = iota
	H2
	H3
	H4
	H5
	H6
	Line
	Invalid
)

type Token struct {
	ttype Type
	content   string
}

func New(ttype Type, content string) Token {
	return Token {
		ttype: ttype,
		content: content,
	}
}

func (self *Token) ToHTML() string {
	switch self.ttype {
	case H1:
		return "<h1>" + self.content + "</h1>"
	case H2:
		return "<h2>" + self.content + "</h2>"
	case H3:
		return "<h3>" + self.content + "</h3>"
	case H4:
		return "<h4>" + self.content + "</h4>"
	case H5:
		return "<h5>" + self.content + "</h5>"
	case H6:
		return "<h6>" + self.content + "</h6>"
	case Line:
		return "<p><p>"
	case Invalid:
		return "Invalid"
	}

	panic("Reached unreachable code point in ToHTML")
}
