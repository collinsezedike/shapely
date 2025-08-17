import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Shapely } from "../target/types/shapely";
import {
	ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
	SYSTEM_PROGRAM_ADDRESS,
	TOKEN_METADATA_PROGRAM_ADDRESS,
	TOKEN_PROGRAM_ADDRESS,
} from "gill/programs";
import {
	ComputeBudgetProgram,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	Transaction,
} from "@solana/web3.js";

import {
	generateAndAirdropSigner,
	getATA,
	getAvatarMintPDA,
	getCollectionMintPDA,
	getConfigPDA,
	getListingPDA,
	getMasterEdition,
	getMetadataAccount,
	getTreasuryPDA,
} from "./helpers";

// import wallet1 from "/home/collins/.config/solana/id.json";
// import wallet2 from "../test-wallet.json";

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

	let listing: PublicKey;
	let listingVault: PublicKey;
	let artistAccessoryAta: PublicKey;
	let collectorAvatarAta: PublicKey;
	let collectorAccessoryAta: PublicKey;

	const configSeed = Math.floor(Math.random() * 10_000_000_000);
	const fee = 150; // 1.5%

	const avatarName = "AVATAR-#001";
	const avatarURI = "https://www.jsonkeeper.com/b/98WJO";

	const accessoryName = "ACCESSORY-#001";
	const accessoryURI = "https://www.jsonkeeper.com/b/QOVHK";

	before(async () => {
		// For localnet
		payer = await generateAndAirdropSigner(provider.connection);
		artist = await generateAndAirdropSigner(provider.connection);
		collector = await generateAndAirdropSigner(provider.connection);

		// For devnet
		// payer = Keypair.fromSecretKey(Uint8Array.from(wallet1));
		// artist = Keypair.fromSecretKey(Uint8Array.from(wallet1));
		// collector = Keypair.fromSecretKey(Uint8Array.from(wallet2));

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

		listing = await getListingPDA(
			accessoryMint.publicKey,
			artist.publicKey
		);
		listingVault = await getATA(accessoryMint.publicKey, listing);
		artistAccessoryAta = await getATA(
			accessoryMint.publicKey,
			artist.publicKey
		);
		collectorAvatarAta = await getATA(avatarMint, collector.publicKey);
		collectorAccessoryAta = await getATA(
			accessoryMint.publicKey,
			collector.publicKey
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
						accessoryMasterEdition,

						accessoryCollection,
						accessoryCollectionMetadata,
						accessoryCollectionMasterEdition,

						sysvarInstruction:
							anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,

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
						avatarMasterEdition,

						avatarCollection,
						avatarCollectionMetadata,
						avatarCollectionMasterEdition,

						sysvarInstruction:
							anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,

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

	it("Should list an accessory", async () => {
		const accessoryPrice = 0.01 * LAMPORTS_PER_SOL;

		const tx = new Transaction().add(
			await program.methods
				.listAccessory(new BN(accessoryPrice))
				.accountsStrict({
					artist: artist.publicKey,
					artistAccessoryAta,

					config,
					listing,
					listingVault,

					accessoryMint: accessoryMint.publicKey,
					accessoryMetadata,
					accessoryCollection,
					accessoryMasterEdition,

					metadataProgram: TOKEN_METADATA_PROGRAM_ADDRESS,
					tokenProgram: TOKEN_PROGRAM_ADDRESS,
					associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
					systemProgram: SYSTEM_PROGRAM_ADDRESS,
				})
				.instruction()
		);

		const sig = await provider.sendAndConfirm(tx, [artist]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});

	it("Should delist an accessory", async () => {
		const tx = new Transaction().add(
			await program.methods
				.delistAccessory()
				.accountsStrict({
					artist: artist.publicKey,
					artistAccessoryAta,

					config,
					listing,
					listingVault,

					accessoryMint: accessoryMint.publicKey,

					tokenProgram: TOKEN_PROGRAM_ADDRESS,
					systemProgram: SYSTEM_PROGRAM_ADDRESS,
				})
				.instruction()
		);

		const sig = await provider.sendAndConfirm(tx, [artist]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});

	it("Should relist an accessory", async () => {
		const accessoryPrice = 0.01 * LAMPORTS_PER_SOL;

		const tx = new Transaction().add(
			await program.methods
				.listAccessory(new BN(accessoryPrice))
				.accountsStrict({
					artist: artist.publicKey,
					artistAccessoryAta,

					config,
					listing,
					listingVault,

					accessoryMint: accessoryMint.publicKey,
					accessoryMetadata,
					accessoryCollection,
					accessoryMasterEdition,

					metadataProgram: TOKEN_METADATA_PROGRAM_ADDRESS,
					tokenProgram: TOKEN_PROGRAM_ADDRESS,
					associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
					systemProgram: SYSTEM_PROGRAM_ADDRESS,
				})
				.instruction()
		);

		const sig = await provider.sendAndConfirm(tx, [artist]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});

	it("Should buy a listed accessory", async () => {
		const tx = new Transaction().add(
			await program.methods
				.buyAccessory()
				.accountsStrict({
					collector: collector.publicKey,
					collectorAccessoryAta,

					config,
					treasury,
					listing,
					listingVault,

					artist: artist.publicKey,
					accessoryMint: accessoryMint.publicKey,

					tokenProgram: TOKEN_PROGRAM_ADDRESS,
					associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
					systemProgram: SYSTEM_PROGRAM_ADDRESS,
				})
				.instruction()
		);

		const sig = await provider.sendAndConfirm(tx, [collector]);

		console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
	});
});
