//https://www.digitalocean.com/community/tutorials/how-to-make-an-http-server-in-go
//https://blog.logrocket.com/creating-a-web-server-with-golang/

package main

// this is the fancy backend, made to be used with the terminal in the devcontainer

import (
	"net/http"
	"log"
	"os"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080" // valor padrão
	}

    log.Printf("The server has successfully initialized at http://localhost:%s\n", port)

	// File handler
	fileServer := http.FileServer(http.Dir("../static"))
    http.Handle("/", fileServer)

	// Start the server
	if err := http.ListenAndServe(":"+port, nil); err != nil {
        log.Printf("Error initializing the server: %v\n", err)
    }
}
