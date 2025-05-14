BINARY = ./cmd/mdb
TARGET = mdb

run:
	go run $(BINARY) $(ARGS)

fmt:
	go fmt ./...

test:
	go test ./...

release:
	mkdir release -p
	GOOS=linux GOARCH=amd64 go build -ldflags="-s -w" -o release/$(TARGET)-linux-amd64 $(BINARY)
	GOOS=darwin GOARCH=arm64 go build -ldflags="-s -w" -o release/$(TARGET)-darwin-arm64 $(BINARY)
	GOOS=darwin GOARCH=amd64 go build -ldflags="-s -w" -o release/$(TARGET)-darwin-arm64 $(BINARY)
	GOOS=windows GOARCH=amd64 go build -ldflags="-s -w" -o release/$(TARGET)-windows-amd64.exe $(BINARY)
