import express from "express";
import bodyParser from "body-parser";

const app = express();
const port = 3000;
const users = [];

app.use(bodyParser.json());

//_____________________________________________________________________________

app.get(
  "/", 
  (req, res) => {
    res.send("Hello world");
  }
)

app.get(
  "/users", 
  (req, res) => {
    // This is shorthand for: res.json({users: users});
    res.json({users});
  }
)


//_____________________________________________________________________________

// The client sends a request to add a new userId to the users variable,
// which will act as a mock database.
app.post(
  "/users",
  (req, res) => {
    // Get the `userId` value from the body of the clients response
    const newUserId = req.body.userId;

    if (!newUserId) {
      return res.status(400).send("Error: userId is missing from this POST request");
    }

    if (users.includes(newUserId)) {
      return res.status(400).send("Error: userId is already exists");
    }

    users.push(newUserId);
    return res.status(201).send("User registered");

  }
)

//_____________________________________________________________________________

app.listen(
  port, 
  () => {
    console.log(`Server started on port: ${port}`);
  }
);

//_____________________________________________________________________________
