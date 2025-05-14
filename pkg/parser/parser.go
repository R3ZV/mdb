package parser

import (
	"log/slog"
	"mdb/pkg/token"
	"strings"
)

func Parse(content string) []token.Token {
	tokens := []token.Token{}
	slog.Info("Parsing: ", "content", content)

	for line := range strings.Lines(content) {
		line := strings.Trim(line, " \n\r\t")

		currToken := token.New(token.Invalid, "")
		if len(line) == 0 {
			currToken = token.New(token.Line, "")
		} else if line[0] == '#' {
			currToken = parseHeader(line)
		}

		slog.Info("Parsed: ", "line", line, "token", currToken)
		tokens = append(tokens, currToken)
	}
	return tokens
}

func parseHeader(line string) token.Token {
	level := 0
	i := 0
	for i < len(line) {
		if line[i] == '#' {
			level += 1
		} else {
			break
		}

		i += 1
	}

	content := strings.TrimSpace(line[i:])

	return token.New(token.Type(level - 1), content)
}
