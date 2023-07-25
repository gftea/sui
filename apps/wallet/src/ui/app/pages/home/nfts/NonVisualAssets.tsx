// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiObjectData } from '@mysten/sui.js';
import { formatAddress } from '@mysten/sui.js/utils';
import ExplorerLink from '_src/ui/app/components/explorer-link';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { Text } from '_src/ui/app/shared/text';

export default function NonVisualAssets({ items }: { items: SuiObjectData[] }) {
	return (
		<div className="flex flex-1 flex-col flex-nowrap items-center gap-4 w-full">
			{items?.length ? (
				<div className="flex flex-col flex-wrap w-full divide-y divide-solid divide-gray-40 divide-x-0 gap-3 mb-5">
					{items.map((item) => {
						return (
							<div
								className="flex justify-between items-center pt-3 flex-nowrap"
								key={item.objectId}
							>
								<ExplorerLink
									className="text-hero-dark no-underline basis-1/2"
									objectID={item.objectId!}
									type={ExplorerLinkType.object}
								>
									<Text variant="pBody">{formatAddress(item.objectId!)}</Text>
								</ExplorerLink>

								<div className="basis-1/2 grow-0 overflow-hidden">
									<Text variant="pBodySmall" mono color="steel" truncate title={item.type}>
										{item.type}
									</Text>
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
