package main

import (
    "github.com/gin-gonic/gin"
)

func main() {

    // This creates an Engine struct using the third-party package gin.
    // This Engine struct is basically an HTTP server that can listen
    // for requests on a specific address.
    // The Engine struct also contains some useful features like a `Logger`,
    // to log requests (useful for debugging and data analytics), 
    // and a `Recovery` middleware that makes the server resilient against
    // crashes.

    var server *gin.Engine = gin.Default() 

    // This will start the server and listen for connections at:
    // http://127.0.0.1:8080

    // 127.0.0.1 is the local network on your computer that is not exposed
    // to the internet. This is often used for testing.
    server.Run("127.0.0.1:8080") 

    // You should get this message:
    // 404 page not found

    // This is because you have to set up a `handler` for that route.
    // A `handler` is basically an instruction (usually a function) 
    // for what the server should do when a request is made 
    // to one of its routes.

    // Since you have not given the server any instructions, 
    // it is responding with a default plain text response of 
    // "404 page not found"
}
