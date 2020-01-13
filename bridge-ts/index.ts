import express from "express"

const app = express()
const port = 8080
const host = "localhost"

app.get("/", (req, res) => {
	res.send("Nodle Chain bridge")
})

app.listen(port, host, () => {
	console.log(`Listening on http://${host}:${port}...`);
})