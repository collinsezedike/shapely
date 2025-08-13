import {
	Address,
	getAddressEncoder,
	createSolanaClient,
	getProgramDerivedAddress,
	createTransaction,
	TransactionSigner,
	Instruction,
	signTransactionMessageWithSigners,
	getExplorerLink,
	getSignatureFromTransaction,
} from "gill";

const addressEncoder = getAddressEncoder();

export function getSolanaClient() {
	const { rpc, sendAndConfirmTransaction } = createSolanaClient({
		urlOrMoniker: "devnet",
	});
	return { rpc, sendAndConfirmTransaction };
}

export async function submitTransaction(
	payer: TransactionSigner,
	ixn: Instruction
) {
	const { value: latestBlockhash } = await getSolanaClient()
		.rpc.getLatestBlockhash()
		.send();

	const tx = createTransaction({
		feePayer: payer,
		version: "legacy",
		instructions: [ixn],
		latestBlockhash,
	});

	const signedTransaction = await signTransactionMessageWithSigners(tx);

	console.log(
		"Explorer:",
		getExplorerLink({
			cluster: "devnet",
			transaction: getSignatureFromTransaction(signedTransaction),
		})
	);

	await getSolanaClient().sendAndConfirmTransaction(signedTransaction);
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
