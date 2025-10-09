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
	httpStatusCode := http.StatusOK

	// This will use the `GetAllEvents` function from the models package,
	// to view the contents of the `events` variable from the file event.go
	// events is a list of Event structs
	dataForResponse := models.GetAllEvents()

	// context.JSON() will convert the list of Events into JSON which will
	// then be sent back to the client.
	context.JSON(httpStatusCode, dataForResponse)
}

func createEvent(context *gin.Context) {
    // SUB_SECTION: Setup

    // This will create an empty `Event` struct that will store the JSON
    // response from the client to the server.
	var event models.Event

    // This will read the JSON data from the client's request body
    // and automatically map (bind) it to the fields of the `event` struct.
    // If the JSON is missing required fields or has invalid types,
    // an error will be returned.

    // If the operation succeeds → err will be nil (as in no errors)
    err := context.ShouldBindJSON(&event)
    
    //_________________________________________________________________________
    // SUB_SECTION: Error Handling

    errorHttpStatusCode := http.StatusBadRequest
    errorJSONResponse := gin.H{
        "message": "Error: Event could not be created",
    }

	if err != nil {
		context.JSON(errorHttpStatusCode, errorJSONResponse)
        return 
        // This will ensure that this function exits if there is an error.
	}
    
    //_________________________________________________________________________
    // SUB_SECTION: Record Success

    // Because of the return statement, if there is an error.

    // These are just a dummy values for now.
    event.ID = 1
    event.UserID = 1

    // Add the new event to the list of events
    event.Save()
    
    successHttpStatusCode := http.StatusCreated
    successJsonResponse := gin.H{
        "message": "Success: Event created",
        "event": event,
    }

    context.JSON(successHttpStatusCode, successJsonResponse)
}

func main() {
	server := gin.Default()

	server.GET("/events", getEvents)

	server.POST("/events", createEvent)

	server.Run("127.0.0.1:8080")
}
