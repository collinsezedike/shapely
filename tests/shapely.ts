import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Shapely } from "../target/types/shapely";
import {
	ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
	getTokenMetadataAddress,
	SYSTEM_PROGRAM_ADDRESS,
	TOKEN_METADATA_PROGRAM_ADDRESS,
	TOKEN_PROGRAM_ADDRESS,
} from "gill/programs";
import {
	Keypair,
	ComputeBudgetProgram,
	Transaction,
	PublicKey,
} from "@solana/web3.js";

import {
	generateAndAirdropSigner,
	getATA,
	getAvatarMintPDA,
	getCollectionMintPDA,
	getConfigPDA,
	getMasterEdition,
	getMetadataAccount,
	getTreasuryPDA,
} from "./helpers";

describe("Shapely", () => {
	const provider = anchor.AnchorProvider.env();

	anchor.setProvider(provider);

	const program = anchor.workspace.Shapely as Program<Shapely>;

	let payer: Keypair;
	let artist: Keypair;
	let collector: Keypair;

	let config: PublicKey;
	let treasury: PublicKey;

	let avatarMint: PublicKey;
	let avatarMetadata: PublicKey;
	let avatarMasterEdition: PublicKey;
	let avatarCollection: PublicKey;
	let avatarCollectionAta: PublicKey;
	let avatarCollectionMetadata: PublicKey;
	let avatarCollectionMasterEdition: PublicKey;

	let accessoryMint: Keypair;
	let accessoryMetadata: PublicKey;
	let accessoryMasterEdition: PublicKey;
	let accessoryCollection: PublicKey;
	let accessoryCollectionAta: PublicKey;
	let accessoryCollectionMetadata: PublicKey;
	let accessoryCollectionMasterEdition: PublicKey;

	let artistAccessoryAta: PublicKey;
	let collectorAvatarAta: PublicKey;

	const configSeed = Math.floor(Math.random() * 10_000_000_000);
	const fee = 150; // 1.5%
	const avatarName = "AVATAR-#001";
	const accessoryName = "ACCESSORY-#001";
	const accessoryURI = "https://www.jsonkeeper.com/b/QOVHK";
	const avatarURI = "https://www.jsonkeeper.com/b/98WJO";

	before(async () => {
		payer = await generateAndAirdropSigner(provider.connection);
		artist = await generateAndAirdropSigner(provider.connection);
		collector = await generateAndAirdropSigner(provider.connection);

		config = await getConfigPDA(configSeed);
		treasury = await getTreasuryPDA(config);

		avatarCollection = await getCollectionMintPDA("avatar", config);
		avatarCollectionAta = await getATA(avatarCollection, config);
		avatarCollectionMetadata = await getMetadataAccount(avatarCollection);
		avatarCollectionMasterEdition =
			await getMasterEdition(avatarCollection);

		avatarMint = await getAvatarMintPDA(
			collector.publicKey,
			avatarCollection
		);
		avatarMetadata = await getMetadataAccount(avatarMint);
		avatarMasterEdition = await getMasterEdition(avatarMint);

		accessoryCollection = await getCollectionMintPDA("accessory", config);
		accessoryCollectionAta = await getATA(accessoryCollection, config);
		accessoryCollectionMetadata =
			await getMetadataAccount(accessoryCollection);
		accessoryCollectionMasterEdition =
			await getMasterEdition(accessoryCollection);

		accessoryMint = Keypair.generate();
		accessoryMetadata = await getMetadataAccount(accessoryMint.publicKey);
		accessoryMasterEdition = await getMasterEdition(
			accessoryMint.publicKey
		);

		artistAccessoryAta = await getATA(
			accessoryMint.publicKey,
			artist.publicKey
		);
		collectorAvatarAta = await getATA(avatarMint, collector.publicKey);
	});

	it("Should initialize the avatar and accessory collection", async () => {
		const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
			units: 400_000,
		});
		const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
			microLamports: 1,
		});

		const tx = new Transaction()
			.add(modifyComputeUnits) // Request higher CU limit
			.add(addPriorityFee) // Optional: offer priority fee
			.add(
				await program.methods
					.initialize(new BN(configSeed), fee)
					.accountsStrict({
						payer: payer.publicKey,
						config,
						treasury,

						avatarCollection,
						avatarCollectionAta,
						avatarCollectionMetadata,
						avatarCollectionMasterEdition,

						accessoryCollection,
						accessoryCollectionAta,
						accessoryCollectionMetadata,
						accessoryCollectionMasterEdition,

						metadataProgram: TOKEN_METADATA_PROGRAM_ADDRESS,
						tokenProgram: TOKEN_PROGRAM_ADDRESS,
						associatedTokenProgram:
							ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
						systemProgram: SYSTEM_PROGRAM_ADDRESS,
					})
					.instruction()
			);

		const sig = await provider.sendAndConfirm(tx, [payer]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});

	it("Should initialize a new accessory mint", async () => {
		const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
			units: 400_000,
		});
		const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
			microLamports: 1,
		});

		const tx = new Transaction()
			.add(modifyComputeUnits) // Request higher CU limit
			.add(addPriorityFee) // Optional: offer priority fee
			.add(
				await program.methods
					.mintAccessory(accessoryName, accessoryURI)
					.accountsStrict({
						artist: artist.publicKey,
						artistAccessoryAta,

						config,

						accessoryMint: accessoryMint.publicKey,
						accessoryMetadata,
						accessoryCollection,
						accessoryMasterEdition,

						metadataProgram: TOKEN_METADATA_PROGRAM_ADDRESS,
						tokenProgram: TOKEN_PROGRAM_ADDRESS,
						associatedTokenProgram:
							ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
						systemProgram: SYSTEM_PROGRAM_ADDRESS,
					})
					.instruction()
			);

		const sig = await provider.sendAndConfirm(tx, [artist, accessoryMint]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});

	it("Should initialize a new avatar mint", async () => {
		const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
			units: 400_000,
		});
		const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
			microLamports: 1,
		});

		const tx = new Transaction()
			.add(modifyComputeUnits) // Request higher CU limit
			.add(addPriorityFee) // Optional: offer priority fee
			.add(
				await program.methods
					.mintAvatar(avatarName, avatarURI)
					.accountsStrict({
						collector: collector.publicKey,
						collectorAvatarAta,

						config,

						avatarMint,
						avatarMetadata,
						avatarCollection,
						avatarMasterEdition,

						metadataProgram: TOKEN_METADATA_PROGRAM_ADDRESS,
						tokenProgram: TOKEN_PROGRAM_ADDRESS,
						associatedTokenProgram:
							ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
						systemProgram: SYSTEM_PROGRAM_ADDRESS,
					})
					.instruction()
			);

		const sig = await provider.sendAndConfirm(tx, [collector]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});
});
