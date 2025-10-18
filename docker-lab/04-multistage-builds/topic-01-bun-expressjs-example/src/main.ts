import express from "express";

const app = express();

const port = process.env.PORT; 

app.get("/", (req, res) => {
  res.send("Hello from Bun and Express.js");
});

app.listen(port, () => {
  console.log(`Server listening on port: ${port}`);
});
