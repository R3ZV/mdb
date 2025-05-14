package parser

import (
	"testing"
	"mdb/pkg/token"
)

func TestParseHeader(t *testing.T) {
	headers := [...]string{
		"# Header 1",
		"## Header 2",
		"### Header 3",
		"#### Header 4",
		"##### Header 5",
		"###### Header 6",
	}

	expected := [...]token.Token{
		token.New(token.H1, "Header 1"),
		token.New(token.H2, "Header 2"),
		token.New(token.H3, "Header 3"),
		token.New(token.H4, "Header 4"),
		token.New(token.H5, "Header 5"),
		token.New(token.H6, "Header 6"),
	}

	if len(expected) != len(headers) {
		t.Errorf("[DEV ERR]: missmatch between number of expected tokens and headers")
	}

	for i, header := range headers {
		got := parseHeader(header)

		if expected[i] != got {
			t.Errorf("\nExpected header token to be: %v\nBut got: %v\n", expected[i], got)
		}
	}

}

func TestParseHeader1(t *testing.T) {
	headers := [...]string{
		"# Header 1",
	}

	expected := [...]token.Token{
		token.New(token.H1, "Header 1"),
	}

	if len(expected) != len(headers) {
		t.Errorf("[DEV ERR]: missmatch between number of expected tokens and headers")
	}

	for i, header := range headers {
		got := parseHeader(header)

		if expected[i] != got {
			t.Errorf("\nExpected header token to be: %v\nBut got: %v\n", expected[i], got)
		}
	}

}
