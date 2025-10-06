package main

import (
    // Go's built in HTTP module 
	"net/http"
	
    // Third Party Package: A web framework written in Go
	"github.com/gin-gonic/gin"
)

func getEvents(context *gin.Context) {
    // This will send back data that is formatted as JSON
    // This JSON function accepts and HTTP status code,
    // and it also accepts the data that you want to send back to the client

    // 200 status code means everything worked.
    // http.StatusOK = 200
    // This is a map in Go
    context.JSON(http.StatusOK, gin.H{"message": "Hello!"})
}

func main() {
    // This will create an HTTP server
    server := gin.Default()

    // This accepts a url, and the name of a function 
    // that will handle the GET request
    server.GET("/events", getEvents)

    // This will start the sever and start listening for HTTP requests.
    server.Run(":8080") // localhost:8080
}
