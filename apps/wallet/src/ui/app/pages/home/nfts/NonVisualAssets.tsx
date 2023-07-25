// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getKioskIdFromOwnerCap, isKioskOwnerToken } from '@mysten/core';
import { EyeClose16 } from '@mysten/icons';
import { type SuiObjectData } from '@mysten/sui.js';
import { Link } from 'react-router-dom';
import { ErrorBoundary } from '_components/error-boundary';
import { NFTDisplayCard } from '_components/nft-display';
import { Button } from '_src/ui/app/shared/ButtonUI';

export default function NonVisualAssets({ items }: { items: SuiObjectData[] }) {
	return (
		<div className="flex flex-1 flex-col flex-nowrap items-center gap-4 w-full">
			{items?.length ? (
				<div className="flex flex-col w-full divide-y divide-solid divide-gray-40 divide-x-0 gap-2 mb-5">
					{items.map((nft) => {
						return (
							<div className="flex justify-between items-center pt-2 pr-1" key={nft.objectId}>
								<Link
									to={
										isKioskOwnerToken(nft)
											? `/kiosk?${new URLSearchParams({
													kioskId: getKioskIdFromOwnerCap(nft),
											  })}`
											: `/nft-details?${new URLSearchParams({
													objectId: nft.objectId,
											  }).toString()}`
									}
									onClick={() => {}}
									className="no-underline relative truncate"
								>
									<ErrorBoundary>
										<NFTDisplayCard
											objectId={nft.objectId}
											size="xs"
											showLabel
											orientation="horizontal"
										/>
									</ErrorBoundary>
								</Link>
								<div className="h-8 w-8">
									<Button
										variant="secondarySui"
										size="icon"
										onClick={() => {}}
										after={<EyeClose16 />}
									/>
								</div>
							</div>
						);
					})}
				</div>
			) : (
				<div className="flex flex-1 items-center self-center text-caption font-semibold text-steel-darker">
					No NFTs found
				</div>
			)}
		</div>
	);
}
