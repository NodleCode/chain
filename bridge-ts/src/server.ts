import express from "express"

const port = 8080
const host = "localhost"

export function startServer(chainApi: any) {
	const app = express()
	app.get("/status", (req, res) => {
		//
	})
	
	app.listen(port, host, () => {
		console.log(`Listening on http://${host}:${port}`)
	})
}