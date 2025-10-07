package main

import (
	//_________________________________________________________________________
	// SUB_SECTION: Built-In Go modules

	// Go's built in HTTP module
	"net/http"

	//_________________________________________________________________________
	// SUB_SECTION: Developer-Created modules

	// Contains the structure of an event and event related functions
	"rest-api/models"

	//_________________________________________________________________________
	// SUB_SECTION: Third Party modules

	// A web framework written in Go
	"github.com/gin-gonic/gin"
	//_________________________________________________________________________
)

func getEvents(context *gin.Context) {

    events := models.GetAllEvents()

    // This will send back data that is formatted as JSON
    // This JSON function accepts and HTTP status code,
    // and it also accepts the data that you want to send back to the client

    // 200 status code means everything worked.
    // http.StatusOK = 200
    // This is a map in Go
    // context.JSON(http.StatusOK, gin.H{"message": "Hello!"})

    // 
    context.JSON(http.StatusOK, events)
}

func createEvent(context *gin.Context) {

}

func main() {
    // This will create an HTTP server
    server := gin.Default()

    // This accepts a url, and the name of a function 
    // that will handle the GET request
    server.GET("/events", getEvents)

    server.POST("/events", createEvent)

    // This will start the sever and start listening for HTTP requests.
    server.Run(":8080") // localhost:8080
}
