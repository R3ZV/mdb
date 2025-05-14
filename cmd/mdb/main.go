package main

import (
	"fmt"
	"os"
	"mdb/pkg/preview"
)

func help() {
	fmt.Println("Usage: mdb <command>")
	fmt.Println("")
	fmt.Println("Commands:")
	fmt.Println("  gen  - goes into articles and generates posts")
	fmt.Println("  prev - spins up a server for live preview of the parser")
	fmt.Println("  help - prints this message")
}

func genPosts() {
	fmt.Println("todo")
}

func main() {
	if len(os.Args) != 2 {
		fmt.Println("Invalid number of arguments\nUse 'mdb help'")
		return
	}

	command := os.Args[1]
	if command == "gen" {
		genPosts()
	} else if command == "prev" {
		preview.StartPreview()
	} else if command == "help" {
		help();
	} else {
		fmt.Println("Unsupported command\n Use 'mdb help'")
	}
}
