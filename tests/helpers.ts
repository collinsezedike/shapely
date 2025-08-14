import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Shapely } from "../target/types/shapely";
import {
	Address,
	getAddressEncoder,
	getProgramDerivedAddress,
	address,
} from "gill";
import {
	getAssociatedTokenAccountAddress,
	getTokenMetadataAddress,
	TOKEN_METADATA_PROGRAM_ADDRESS,
} from "gill/programs";
import {
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
} from "@solana/web3.js";

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

export async function getATA(
	mint: PublicKey,
	owner: PublicKey
): Promise<PublicKey> {
	const ata = await getAssociatedTokenAccountAddress(
		address(mint.toBase58()),
		address(owner.toBase58())
	);

	return new PublicKey(ata);
}

export async function getMetadataAccount(mint: PublicKey): Promise<PublicKey> {
	const metadata = await getTokenMetadataAddress(address(mint.toBase58()));
	return new PublicKey(metadata);
}

export async function getConfigPDA(seed: number): Promise<PublicKey> {
	const seedBuffer = Buffer.alloc(8);
	seedBuffer.writeBigUInt64LE(BigInt(seed), 0);

	const [configPDA] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: ["config", seedBuffer],
	});

	return new PublicKey(configPDA);
}

export async function getTreasuryPDA(config: PublicKey): Promise<PublicKey> {
	const [treasury] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: ["treasury", addressEncoder.encode(address(config.toBase58()))],
	});

	return new PublicKey(treasury);
}

export async function getCollectionMintPDA(
	collectionType: "avatar" | "accessory",
	config: PublicKey
): Promise<PublicKey> {
	const [collectionMintPDA] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: [
			`${collectionType} collection`,
			addressEncoder.encode(address(config.toBase58())),
		],
	});

	return new PublicKey(collectionMintPDA);
}

export async function getMasterEdition(mint: PublicKey): Promise<PublicKey> {
	const [masterEdition] = await getProgramDerivedAddress({
		programAddress: TOKEN_METADATA_PROGRAM_ADDRESS,
		seeds: [
			"metadata",
			addressEncoder.encode(TOKEN_METADATA_PROGRAM_ADDRESS),
			addressEncoder.encode(address(mint.toBase58())),
			"edition",
		],
	});

	return new PublicKey(masterEdition);
}

export async function getAvatarMintPDA(
	collector: PublicKey,
	avatarCollection: PublicKey
): Promise<PublicKey> {
	const [NFTMintPDA] = await getProgramDerivedAddress({
		programAddress: PROGRAM_ID,
		seeds: [
			"avatar",
			addressEncoder.encode(address(collector.toBase58())),
			addressEncoder.encode(address(avatarCollection.toBase58())),
		],
	});

	return new PublicKey(NFTMintPDA);
}
