package main

import (
	"fmt"
	"log"
	"net/http"
	"os/exec"
	"strings"
)

func execute_hour() {
	output, err := exec.Command("/usr/local/bin/kiel", "hour-force", "--enact").Output()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(output)
}

func main() {
	// fmt.Println("Ello")
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		if strings.HasPrefix(path, "/service/") {
			echo := strings.Split(path, "/")[2]
			w.WriteHeader(200)
			fmt.Fprintf(w, "Kiel says hello, %s!", echo)
		} else if strings.HasPrefix(path, "/hour") {
			fmt.Println("Hour executed, just kidding")
			execute_hour()
		}
	})

	http.ListenAndServe(":8080", nil)
}
