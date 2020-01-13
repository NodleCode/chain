import express from "express"
import { Keyring } from '@polkadot/api'
import { cryptoWaitReady } from '@polkadot/util-crypto'

const port = process.env.PORT || "8080"
const host = process.env.HOST || "localhost"

const oracleSk = process.env.ORACLE || ""
const hasOracleSk = oracleSk.length > 0

export async function startServer(chainApi: any) {
	const app = express()

	// Oracle only calls
	if (hasOracleSk) {
		await cryptoWaitReady()
		const keyring = new Keyring({ type: 'sr25519' })
		const oraclePair = keyring.addFromUri(oracleSk)

		console.log(`Oracle has address ${oraclePair.address}`)

		app.post("/oracle/reward", (req, res) => {

		})
	}
	
	app.listen(parseInt(port, 10), host, () => {
		console.log(`Listening on http://${host}:${port}`)
	})
}