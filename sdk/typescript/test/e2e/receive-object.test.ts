// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll, beforeEach } from 'vitest';

import {
	getExecutionStatusType,
	getObjectId,
	getTransactionDigest,
	ObjectId,
	TransactionBlock,
	getCreatedObjects,
	getObjectOwner,
	OwnedObjectRef,
	ObjectOwner,
	Keypair,
	getSharedObjectInitialVersion,
} from '../../src';
import { setup, TestToolbox, publishPackage } from './utils/setup';
import { SuiClient } from '../../src/client';

function getOwnerAddress(o: ObjectOwner): string | undefined {
	const owner = getObjectOwner(o);
	if (typeof owner == 'object' && 'AddressOwner' in owner) {
		return owner.AddressOwner;
	} else {
		return undefined;
	}
}

describe('Transfer to Object', () => {
	let toolbox: TestToolbox;
	let packageId: ObjectId;
	let parentObjectId: OwnedObjectRef;
	let receiveObjectId: OwnedObjectRef;
	let sharedObjectId: ObjectId;

	beforeAll(async () => {
		const packagePath = __dirname + '/./data/tto';
		({ packageId } = await publishPackage(packagePath));
	});

	beforeEach(async () => {
		toolbox = await setup();
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::tto::start`,
			typeArguments: [],
			arguments: [],
		});
		const x = await validateTransaction(toolbox.client, toolbox.keypair, tx);
		const y = getCreatedObjects(x)!.map((o) => getOwnerAddress(o.owner))!;
		receiveObjectId = getCreatedObjects(x)!.filter(
			(o) => !y.includes(getObjectId(o)) && !getSharedObjectInitialVersion(o.owner),
		)[0];
		parentObjectId = getCreatedObjects(x)!.filter(
			(o) => y.includes(getObjectId(o)) && !getSharedObjectInitialVersion(o.owner),
		)[0];
		const sharedObject = getCreatedObjects(x)!.filter(
			(o) => getSharedObjectInitialVersion(o.owner) !== undefined,
		)[0];
		sharedObjectId = getObjectId(sharedObject);
	});

	it('Basic Receive: receive and then transfer', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::tto::receiver`,
			typeArguments: [],
			arguments: [
				tx.object(parentObjectId.reference.objectId),
				tx.object(receiveObjectId.reference.objectId),
			],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('Basic Receive: receive and then delete', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::tto::deleter`,
			typeArguments: [],
			arguments: [
				tx.object(parentObjectId.reference.objectId),
				tx.object(receiveObjectId.reference.objectId),
			],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('receive + return, then delete', async () => {
		const tx = new TransactionBlock();
		const b = tx.moveCall({
			target: `${packageId}::tto::return_`,
			typeArguments: [],
			arguments: [
				tx.object(parentObjectId.reference.objectId),
				tx.object(receiveObjectId.reference.objectId),
			],
		});
		tx.moveCall({
			target: `${packageId}::tto::delete_`,
			typeArguments: [],
			arguments: [b],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('Basic Receive: &Receiving arg type', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::tto::invalid_call_immut_ref`,
			typeArguments: [],
			arguments: [
				tx.object(parentObjectId.reference.objectId),
				tx.object(receiveObjectId.reference.objectId),
			],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('Basic Receive: &mut Receiving arg type', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::tto::invalid_call_mut_ref`,
			typeArguments: [],
			arguments: [
				tx.object(parentObjectId.reference.objectId),
				tx.object(receiveObjectId.reference.objectId),
			],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it.fails('Trying to pass shared object as receiving argument', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::tto::receiver`,
			typeArguments: [],
			arguments: [tx.object(parentObjectId.reference.objectId), tx.object(sharedObjectId)],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});
});

async function validateTransaction(client: SuiClient, signer: Keypair, tx: TransactionBlock) {
	tx.setSenderIfNotSet(signer.getPublicKey().toSuiAddress());
	const localDigest = await tx.getDigest({ client });
	const result = await client.signAndExecuteTransactionBlock({
		signer,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
	expect(localDigest).toEqual(getTransactionDigest(result));
	expect(getExecutionStatusType(result)).toEqual('success');
	return result;
}
