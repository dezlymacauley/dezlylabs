import express from "express"

// Create an express application
const app = express()

app.get("/health", 
  (req, res) => {
    res.json({message: "hello"}).status(200)
  }
);

export { app }

export default app
