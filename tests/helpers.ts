import {
	Address,
	getAddressEncoder,
	createSolanaClient,
	generateKeyPairSigner,
	getExplorerLink,
	getProgramDerivedAddress,
	KeyPairSigner,
	lamports,
	LAMPORTS_PER_SOL,
} from "gill";

const addressEncoder = getAddressEncoder();

export function getSolanaClient() {
	const { rpc, sendAndConfirmTransaction } = createSolanaClient({
		urlOrMoniker: "localnet",
	});
	return { rpc, sendAndConfirmTransaction };
}

export async function generateAndAirdropKeypairSigner(): Promise<KeyPairSigner> {
	const keypair = await generateKeyPairSigner();

	const sig = await getSolanaClient()
		.rpc.requestAirdrop(
			keypair.address,
			lamports(BigInt(5 * LAMPORTS_PER_SOL))
		)
		.send();

	// Wait 2 seconds for the transaction to be fulfilled?
	await new Promise((resolve) => setTimeout(resolve, 2000));

	// console.log("Airdrop Explorer:", getExplorerLink({ cluster: "localnet", transaction: sig }));

	const kpBalance = (await getSolanaClient().rpc.getBalance(keypair.address).send());

	console.log("Keypair balance:", kpBalance)

	return keypair;
}

export async function getConfigPDA(
	programAddress: Address,
	seed: number
): Promise<Address> {
	const seedBuffer = Buffer.alloc(8);
	seedBuffer.writeBigUInt64LE(BigInt(seed), 0);

	const [configPDA] = await getProgramDerivedAddress({
		programAddress,
		seeds: ["config", seedBuffer],
	});

	return configPDA;
}

export async function getTreasuryPDA(
	programAddress: Address,
	config: Address
): Promise<Address> {
	const [treasury] = await getProgramDerivedAddress({
		programAddress,
		seeds: ["treasury", addressEncoder.encode(config)],
	});

	return treasury;
}
