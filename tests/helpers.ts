import {
	Address,
	getAddressEncoder,
	createSolanaClient,
	generateKeyPairSigner,
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

	await getSolanaClient()
		.rpc.requestAirdrop(
			keypair.address,
			lamports(BigInt(5 * LAMPORTS_PER_SOL))
		)
		.send();

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

export async function getCollectionPDA(
	collection: string,
	programAddress: Address,
	config: Address
): Promise<Address> {
	const [collectionPDA] = await getProgramDerivedAddress({
		programAddress,
		seeds: [`${collection} collection`, addressEncoder.encode(config)],
	});

	return collectionPDA;
}
