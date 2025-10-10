package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func getEvents(context *gin.Context) {
    // An HTTP status code is a three-digit number sent by a web server 
    // in response to a client’s request (like a browser 
    // or an API client). 
    // It tells the client what happened with its request.
    // 200 means "OK", as in the server was succesfully able to get a
    // response back from the client.

    // NOTE: 200 means that the server successfully received, understood, 
    // and processed the client's request. 
    // It does not guarantee that the response content 
    // is correct for your application logic.

    // Go has a built in http module so that you don't have to remember
    // the number of each HTTP status code.

    // This is the same as: 
    // httpStatusCode = 200
    httpStatusCode := http.StatusOK

    jsonResponse := map[string]string{
        // key: value
        "message": "hello",
    }

    // `.JSON` converts the `jsonResponse` map into JSON
    // and sends it back to the client with the given status code.
    context.JSON(httpStatusCode, jsonResponse)
}

func main() {
	server := gin.Default()

    // `context` is a pointer to an instance of the Context struct from
    // the `gin` package.
    // It contains information about the incoming request (headers, query params, 
    // body, etc.) from the client,
    // and it also provides methods (like `.JSON`, `.String`, `.HTML`) 
    // to send a response back to the client.

    // In this example, I am dealing with a simple GET request, so I don't
    // need to read the data from the client, I just need to send JSON
    // data back to the client.
	server.GET("/events", getEvents)

	server.Run("127.0.0.1:8080")
    /*

        {
            "message": "hello"
        }

    */

}
