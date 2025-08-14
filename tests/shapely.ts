import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Shapely } from "../target/types/shapely";
import { Address } from "gill";
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

	let config: Address;
	let treasury: Address;

	let avatarCollection: Address;
	let avatarCollectionAta: Address;
	let avatarCollectionMetadata: Address;
	let avatarCollectionMasterEdition: Address;

	let accessoryCollection: Address;
	let accessoryCollectionAta: Address;
	let accessoryCollectionMetadata: Address;
	let accessoryCollectionMasterEdition: Address;

	let configSeed = Math.floor(Math.random() * 10_000_000_000);
	let fee = 150; // 1.5%

	before(async () => {
		payer = await generateAndAirdropSigner(provider.connection);

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

		accessoryCollection = await getCollectionMintPDA("accessory", config);
		accessoryCollectionAta = await getAssociatedTokenAccountAddress(
			accessoryCollection,
			config
		);
		accessoryCollectionMetadata =
			await getTokenMetadataAddress(accessoryCollection);
		accessoryCollectionMasterEdition =
			await getMasterEdition(accessoryCollection);
	});

	it("Should initialize the collection mints", async () => {
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
});
