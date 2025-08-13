import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import {
	address,
	Address,
	createTransaction,
	generateKeyPairSigner,
	getExplorerLink,
	getSignatureFromTransaction,
	KeyPairSigner,
	signTransactionMessageWithSigners,
} from "gill";
import { SYSTEM_PROGRAM_ADDRESS } from "gill/programs";

import * as programClient from "../client/ts";
import { getInitializeInstruction } from "../client/ts";

import {
	generateAndAirdropKeypairSigner,
	getCollectionPDA,
	getConfigPDA,
	getSolanaClient,
	getTreasuryPDA,
} from "./helpers";
import { generateKeyPair } from "node:crypto";

type initializeParams = Parameters<typeof getInitializeInstruction>[0];

describe("Shapely", () => {
	const PROGRAM_ID = programClient.SHAPELY_PROGRAM_ADDRESS;
	const MPL_PROGRAM_ID = address(
		"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
	);

	let marketMaker: KeyPairSigner;
	// let avatarCollection: KeyPairSigner;
	// let accessoryCollection: KeyPairSigner;
	let avatarCollection: Address;
	let accessoryCollection: Address;
	let config: Address;
	let treasury: Address;

	let configSeed = Math.floor(Math.random() * 10_000_000_000);
	let fee = 150; // 1.5%

	before(async () => {
		marketMaker = await generateAndAirdropKeypairSigner();
		// avatarCollection = await generateKeyPairSigner();
		// accessoryCollection = await generateKeyPairSigner();
		avatarCollection = await getCollectionPDA("avatar", PROGRAM_ID, config);
		accessoryCollection = await getCollectionPDA(
			"accessory",
			PROGRAM_ID,
			config
		);

		config = await getConfigPDA(PROGRAM_ID, configSeed);
		treasury = await getTreasuryPDA(PROGRAM_ID, config);
	});

	it("Should initialize the collection mints", async () => {
		const params: initializeParams = {
			seed: configSeed,
			fee,
			marketMaker,
			accessoryCollection,
			avatarCollection,
			config,
			treasury,
			systemProgram: SYSTEM_PROGRAM_ADDRESS,
			mplCoreProgram: MPL_PROGRAM_ID,
		};

		const ixn = getInitializeInstruction(params);

		const { value: latestBlockhash } = await getSolanaClient()
			.rpc.getLatestBlockhash()
			.send();

		const tx = createTransaction({
			feePayer: marketMaker,
			version: "legacy",
			instructions: [ixn],
			latestBlockhash,
		});

		const signedTransaction = await signTransactionMessageWithSigners(tx);

		console.log(
			"Explorer:",
			getExplorerLink({
				cluster: "localnet",
				transaction: getSignatureFromTransaction(signedTransaction),
			})
		);

		await getSolanaClient().sendAndConfirmTransaction(signedTransaction);
	});
});
