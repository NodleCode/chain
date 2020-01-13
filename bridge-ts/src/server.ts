import express from "express"
import { Keyring } from '@polkadot/api'
import { cryptoWaitReady } from '@polkadot/util-crypto'

const port = 8080
const host = "localhost"

const oracleSk = "0xf66143bce8f196f74cf7686b32afcad8a09a6f6f2d0bf5ef50fb724b6ce37350"
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
	
	app.listen(port, host, () => {
		console.log(`Listening on http://${host}:${port}`)
	})
}