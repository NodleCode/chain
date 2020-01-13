import { ApiPromise, WsProvider } from "@polkadot/api"
import { startServer } from "./src/server"

async function main() {
	const provider = new WsProvider('ws://127.0.0.1:9944')
	const api = await ApiPromise.create({ provider })

	const [chain, nodeName, nodeVersion] = await Promise.all([
		api.rpc.system.chain(),
		api.rpc.system.name(),
		api.rpc.system.version()
	])

	console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`)

	await startServer(api)
}

main().catch(console.error)//.finally(() => process.exit());