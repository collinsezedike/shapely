import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Shapely } from "../target/types/shapely";
import { address, Address } from "gill";
import {
	ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
	getAssociatedTokenAccountAddress,
	getTokenMetadataAddress,
	SYSTEM_PROGRAM_ADDRESS,
	TOKEN_METADATA_PROGRAM_ADDRESS,
	TOKEN_PROGRAM_ADDRESS,
} from "gill/programs";
import { Keypair, ComputeBudgetProgram, Transaction } from "@solana/web3.js";

import {
	generateAndAirdropSigner,
	getAvatarMintPDA,
	getCollectionMintPDA,
	getConfigPDA,
	getMasterEdition,
	getTreasuryPDA,
} from "./helpers";

describe("Shapely", () => {
	const provider = anchor.AnchorProvider.env();

	anchor.setProvider(provider);

	const program = anchor.workspace.Shapely as Program<Shapely>;

	let payer: Keypair;
	let artist: Keypair;
	let collector: Keypair;

	let config: Address;
	let treasury: Address;

	let avatarMint: Address;
	let avatarMetadata: Address;
	let avatarMasterEdition: Address;
	let avatarCollection: Address;
	let avatarCollectionAta: Address;
	let avatarCollectionMetadata: Address;
	let avatarCollectionMasterEdition: Address;

	let accessoryMint: Keypair;
	let accessoryMetadata: Address;
	let accessoryMasterEdition: Address;
	let accessoryCollection: Address;
	let accessoryCollectionAta: Address;
	let accessoryCollectionMetadata: Address;
	let accessoryCollectionMasterEdition: Address;

	let artistAccessoryAta: Address;
	let collectorAvatarAta: Address;

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
		avatarCollectionAta = await getAssociatedTokenAccountAddress(
			avatarCollection,
			config
		);
		avatarCollectionMetadata =
			await getTokenMetadataAddress(avatarCollection);
		avatarCollectionMasterEdition =
			await getMasterEdition(avatarCollection);
		avatarMint = await getAvatarMintPDA(
			address(collector.publicKey.toBase58()),
			avatarCollection
		);
		avatarMetadata = await getTokenMetadataAddress(avatarMint);
		avatarMasterEdition = await getMasterEdition(avatarMint);

		accessoryCollection = await getCollectionMintPDA("accessory", config);
		accessoryCollectionAta = await getAssociatedTokenAccountAddress(
			accessoryCollection,
			config
		);
		accessoryCollectionMetadata =
			await getTokenMetadataAddress(accessoryCollection);
		accessoryCollectionMasterEdition =
			await getMasterEdition(accessoryCollection);
		accessoryMint = Keypair.generate();
		accessoryMetadata = await getTokenMetadataAddress(
			address(accessoryMint.publicKey.toBase58())
		);
		accessoryMasterEdition = await getMasterEdition(
			address(accessoryMint.publicKey.toBase58())
		);

		artistAccessoryAta = await getAssociatedTokenAccountAddress(
			address(accessoryMint.publicKey.toBase58()),
			address(artist.publicKey.toBase58())
		);
		collectorAvatarAta = await getAssociatedTokenAccountAddress(
			avatarMint,
			address(collector.publicKey.toBase58())
		);
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
