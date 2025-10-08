package main

import (
	//_________________________________________________________________________
	// SUB_SECTION: Built-In Go modules

	// Go's built in HTTP module
	"net/http"

	//_________________________________________________________________________
	// SUB_SECTION: Developer-Created modules

	// Contains the structure of an event and event related functions
	// "rest-api/models"

	//_________________________________________________________________________
	// SUB_SECTION: Third Party modules

	// A web framework written in Go
	"github.com/gin-gonic/gin"
	//_________________________________________________________________________
)

func getEvents(context *gin.Context) {
    httpStatusCode := http.StatusOK

    jsonResponse := map[string]string{
        "message": "hello",
    }

    context.JSON(httpStatusCode, jsonResponse)
}

func main() {
	server := gin.Default()

	server.GET("/events", getEvents)

	server.Run("127.0.0.1:8080")
}
