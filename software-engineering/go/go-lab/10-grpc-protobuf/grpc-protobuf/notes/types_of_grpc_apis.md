1. Unary RPC
The simplest gRPC call.

This is a single request followed by a single response.

Good for on-off operations. 

Feteching user profiles, or updating a user profile.
_______________________________________________________________________________
2. Server Streaming RPC

The client sends a single request, 
and the server sends back a stream of responses.

Use case, the server has to send large amounts of data over time.

Log entries and large data sets.

_______________________________________________________________________________
3. Client side streaming

Client streams a stream of messages to the server.

The server responses with a single reply.

Use case the client is sending a large file. Such as a file upload.

_______________________________________________________________________________
4. Biderectional Streaming

The client and the server can stream messages to each other in any order.

Real time interactive communictation. 

E.g. a Chat application.

_______________________________________________________________________________
