const express = require("express");
const bodyParser = require("body-parser");

// This will intialize the express application.
const app = express();

const port = 3000;

app.use(bodyParser.json());

app.get("/", 
  (req, res)  => {
    res.send("Hello world");
  }
);

app.listen(port,
  () => {
    console.log(`Server listening on port ${port}`);
  }
);
