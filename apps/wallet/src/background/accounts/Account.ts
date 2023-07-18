// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiAddress } from '@mysten/sui.js';
import {
	clearEphemeralValue,
	getEphemeralValue,
	setEphemeralValue,
} from '../session-ephemeral-values';
import {
	type StorageEntity,
	getStorageEntity,
	updateStorageEntity,
} from '../storage-entities-utils';
import { type Serializable } from '_src/shared/cryptography/keystore';

export type AccountType = 'mnemonic-derived'; // TODO: | 'imported' | 'ledger' | 'qredo';

export abstract class Account<T extends SerializedAccount, V extends Serializable> {
	readonly id: string;
	readonly type: AccountType;
	readonly address: Promise<SuiAddress>;

	constructor({ id, type }: { id: string; type: AccountType }) {
		this.id = id;
		this.type = type;
		const data = this.getStoredData();
		this.address = data.then(({ address }) => address);
	}

	abstract lock(): Promise<void>;
	abstract isLocked(): Promise<boolean>;
	abstract toUISerialized(): Promise<SerializedUIAccount>;

	protected async getStoredData() {
		const data = await getStorageEntity<T>(this.id);
		if (!data) {
			throw new Error(`Account data not found. (id: ${this.id})`);
		}
		return data;
	}

	updateStoredData(update: Parameters<typeof updateStorageEntity<T>>['1']) {
		return updateStorageEntity<T>(this.id, update);
	}

	getEphemeralValue(): Promise<V | null> {
		return getEphemeralValue<V>(this.id);
	}

	setEphemeralValue(value: V) {
		return setEphemeralValue(this.id, value);
	}

	clearEphemeralValue() {
		return clearEphemeralValue(this.id);
	}
}

export interface SerializedAccount extends StorageEntity {
	readonly storageEntityType: 'account-entity';
	readonly type: AccountType;
	readonly address: SuiAddress;
	readonly publicKey: string | null;
}

export interface SerializedUIAccount {
	readonly type: AccountType;
	readonly address: SuiAddress;
	readonly isLocked: boolean;
	readonly publicKey: string | null;
}

export interface PasswordUnLockableAccount {
	readonly unlockType: 'password';
	passwordUnlock(password: string): Promise<void>;
}

export function isPasswordUnLockable(account: any): account is PasswordUnLockableAccount {
	return (
		'passwordUnlock' in account && 'unlockType' in account && account.unlockType === 'password'
	);
}
