import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import {
	address,
	Address,
	createKeyPairSignerFromBytes,
	generateKeyPairSigner,
	KeyPairSigner,
} from "gill";
import { SYSTEM_PROGRAM_ADDRESS } from "gill/programs";

import * as programClient from "../client/ts";
import {
	getInitializeInstruction,
	getMintAccessoryInstruction,
} from "../client/ts";

import { getConfigPDA, getTreasuryPDA, submitTransaction } from "./helpers";
import wallet from "../test-wallet.json";

type initializeParams = Parameters<typeof getInitializeInstruction>[0];
type mintAccessoryParams = Parameters<typeof getMintAccessoryInstruction>[0];

describe("Shapely", () => {
	const PROGRAM_ID = programClient.SHAPELY_PROGRAM_ADDRESS;
	const MPL_PROGRAM_ID = address(
		"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
	);

	let payer: KeyPairSigner;
	let artist: KeyPairSigner;
	let avatarCollection: KeyPairSigner;
	let accessoryCollection: KeyPairSigner;
	let accessory: KeyPairSigner;
	let config: Address;
	let treasury: Address;

	let configSeed = Math.floor(Math.random() * 10_000_000_000);
	let fee = 150; // 1.5%

	before(async () => {
		payer = await createKeyPairSignerFromBytes(Uint8Array.from(wallet));
		artist = await createKeyPairSignerFromBytes(Uint8Array.from(wallet));
		avatarCollection = await generateKeyPairSigner();
		accessoryCollection = await generateKeyPairSigner();
		accessory = await generateKeyPairSigner();

		config = await getConfigPDA(PROGRAM_ID, configSeed);
		treasury = await getTreasuryPDA(PROGRAM_ID, config);
	});

	it("Should initialize the collection mints", async () => {
		const params: initializeParams = {
			// Arguments
			seed: configSeed,
			fee,
			// Accounts
			payer,
			accessoryCollection,
			avatarCollection,
			config,
			treasury,
			systemProgram: SYSTEM_PROGRAM_ADDRESS,
			mplCoreProgram: MPL_PROGRAM_ID,
		};

		const ixn = getInitializeInstruction(params);

		await submitTransaction(payer, ixn);
	});

	it("Should mint a new accessory NFT", async () => {
		const params: mintAccessoryParams = {
			// Arguments
			name: "Cyan Leather Jacket",
			uri: "https://www.jsonkeeper.com/b/QOVHK",
			// Accounts
			artist,
			accessory,
			config,
			accessoryCollection: accessoryCollection.address,
			systemProgram: SYSTEM_PROGRAM_ADDRESS,
			mplCoreProgram: MPL_PROGRAM_ID,
		};

		const ixn = getMintAccessoryInstruction(params);

		await submitTransaction(artist, ixn);
	});
});
