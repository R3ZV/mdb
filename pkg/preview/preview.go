package preview

import (
	"log/slog"
	"mdb/pkg/parser"
	"net/http"
	"time"
)

func previewMarkdown(w http.ResponseWriter, r *http.Request) {
	http.ServeFile(w, r, "preview/index.html")
}

func parseMarkdown(w http.ResponseWriter, r *http.Request) {
	slog.Info("/parse resquest!")
	err := r.ParseForm()
	if err != nil {
		panic(err)
	}


	html := ""
	for _, token := range parser.Parse(r.FormValue("markdown-content")) {
		html += token.ToHTML()
	}

	slog.Info("Responding /parse with: ", "html", html)
	_, err = w.Write([]byte(html))
	if err != nil {
		panic(err)
	}
}

func StartPreview() {
	server := &http.Server{
		Addr:           ":8080",
		ReadTimeout:    10 * time.Second,
		WriteTimeout:   10 * time.Second,
		MaxHeaderBytes: 1 << 20,
	}

	http.HandleFunc("/", previewMarkdown)
	http.HandleFunc("/parse", parseMarkdown)

	slog.Info("Server listening...")
	slog.Error("Failed due to: ", "error", server.ListenAndServe())
}
