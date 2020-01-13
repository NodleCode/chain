import express from "express"
import bodyParser from "body-parser"

import { Keyring } from '@polkadot/api'
import { cryptoWaitReady } from '@polkadot/util-crypto'

const port = process.env.PORT || "8080"
const host = process.env.HOST || "localhost"

const oracleSk = process.env.ORACLE || ""
const hasOracleSk = oracleSk.length > 0

export async function startServer(chainApi: any) {
	const app = express()
	app.use(bodyParser.urlencoded({ extended: true }))
	app.use(bodyParser.json())
	app.use(bodyParser.raw())

	// Oracle only calls
	if (hasOracleSk) {
		await cryptoWaitReady()
		const keyring = new Keyring({ type: 'sr25519' })
		const oraclePair = keyring.addFromUri(oracleSk)

		console.log(`Oracle has address ${oraclePair.address}`)

		app.post("/oracle/reward", async (req, res) => {
			console.log(`Got reward ${JSON.stringify(req.body)}`)
			const reward = chainApi.tx.allocations.submitReward(
				req.body.rootHash,
				req.body.destination,
				req.body.amount
			)
			const hash = await reward.signAndSend(oraclePair)

			res.send(hash)
		})
	}
	
	app.listen(parseInt(port, 10), host, () => {
		console.log(`Listening on http://${host}:${port}`)
	})
}