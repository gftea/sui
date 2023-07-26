// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { hasDisplayData, isKioskOwnerToken, useGetOwnedObjects } from '@mysten/core';
import { type SuiObjectData } from '@mysten/sui.js/client';

type OwnedAssets = {
	visual: SuiObjectData[];
	other: SuiObjectData[];
	ownerCaps: SuiObjectData[];
};

export enum DisplayTypes {
	visual = 'visual',
	other = 'other',
}

export function useGetNFTs(address?: string | null) {
	const {
		data,
		isLoading,
		error,
		isError,
		isFetchingNextPage,
		hasNextPage,
		fetchNextPage,
		isInitialLoading,
	} = useGetOwnedObjects(
		address,
		{
			MatchNone: [{ StructType: '0x2::coin::Coin' }],
		},
		50,
	);

	const ownedAssets: OwnedAssets = {
		visual: [],
		other: [],
		ownerCaps: [],
	};

	const assets = data?.pages
		.flatMap((page) => page.data)
		.reduce((acc, curr) => {
			if (hasDisplayData(curr) || isKioskOwnerToken(curr))
				acc.visual.push(curr.data as SuiObjectData);
			if (!hasDisplayData(curr)) acc.other.push(curr.data as SuiObjectData);
			return acc;
		}, ownedAssets);

	return {
		data: assets,
		isInitialLoading,
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isLoading: isLoading,
		isError: isError,
		error,
	};
}
