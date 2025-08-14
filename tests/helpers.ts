import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Shapely } from "../target/types/shapely";
import {
	Address,
	getAddressEncoder,
	getProgramDerivedAddress,
	address,
} from "gill";
import { TOKEN_METADATA_PROGRAM_ADDRESS } from "gill/programs";

import { Keypair, LAMPORTS_PER_SOL, Connection } from "@solana/web3.js";

const addressEncoder = getAddressEncoder();

const program = anchor.workspace.Shapely as Program<Shapely>;

const PROGRAM_ID = address(program.programId.toBase58());

export async function generateAndAirdropSigner(
	connection: Connection
): Promise<Keypair> {
	const kp = Keypair.generate();
	const signature = await connection.requestAirdrop(
		kp.publicKey,
		5 * LAMPORTS_PER_SOL
	);
	const blockhash = await connection.getLatestBlockhash();
	await connection.confirmTransaction({
		blockhash: blockhash.blockhash,
		lastValidBlockHeight: blockhash.lastValidBlockHeight,
		signature,
	});

	return kp;
}

export async function getConfigPDA(seed: number): Promise<Address> {
	const seedBuffer = Buffer.alloc(8);
	seedBuffer.writeBigUInt64LE(BigInt(seed), 0);

	const [configPDA] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: ["config", seedBuffer],
	});

	return configPDA;
}

export async function getTreasuryPDA(config: Address): Promise<Address> {
	const [treasury] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: ["treasury", addressEncoder.encode(config)],
	});

	return treasury;
}

export async function getCollectionMintPDA(
	collection: string,
	config: Address
): Promise<Address> {
	const [collectionMintPDA] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: [`${collection} collection`, addressEncoder.encode(config)],
	});

	return collectionMintPDA;
}

export async function getMasterEdition(mint: Address): Promise<Address> {
	const [masterEdition] = await getProgramDerivedAddress({
		programAddress: TOKEN_METADATA_PROGRAM_ADDRESS,
		seeds: [
			"metadata",
			addressEncoder.encode(TOKEN_METADATA_PROGRAM_ADDRESS),
			addressEncoder.encode(mint),
			"edition",
		],
	});

	return masterEdition;
}
