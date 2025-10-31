import express from "express";

const app = express();

// NOTE: This is how you get the value of the `PORT` environment variable
// that is listed in the `Dockerfile`
const port = process.env.PORT;
const appName = process.env.APP_NAME;

app.get(
  "/",
  (req, res) => {
    res.send(`The name of the app is: ${appName}`);
  }
);

app.listen(
  port,
  () => {
    console.log(`Server listening on port: ${port}`);
  }
);
