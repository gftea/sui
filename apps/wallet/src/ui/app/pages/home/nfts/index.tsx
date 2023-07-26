// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useOnScreen } from '@mysten/core';
import { Check12 } from '@mysten/icons';
import { get, set } from 'idb-keyval';
import { useRef, useEffect, useCallback, useState, useMemo } from 'react';
import toast from 'react-hot-toast';
import { useParams } from 'react-router-dom';

import AssetsOptionsMenu from './AssetsOptionsMenu';
import NonVisualAssets from './NonVisualAssets';
import VisualAssets from './VisualAssets';
import { Link as InlineLink } from '../../../shared/Link';
import { Text } from '../../../shared/text';
import { useActiveAddress } from '_app/hooks/useActiveAddress';
import Alert from '_components/alert';
import FiltersPortal from '_components/filters-tags';
import Loading from '_components/loading';
import LoadingSpinner from '_components/loading/LoadingIndicator';
import { setToSessionStorage } from '_src/background/storage-utils';
import { DisplayTypes, useGetNFTs } from '_src/ui/app/hooks/useGetNFTs';
import PageTitle from '_src/ui/app/shared/PageTitle';

const HIDDEN_ASSET_IDS = 'hidden-asset-ids';

function NftsPage() {
	const [internalHiddenAssetIds, internalSetHiddenAssetIds] = useState<string[]>([]);
	const accountAddress = useActiveAddress();
	const {
		data: ownedAssets,
		hasNextPage,
		isInitialLoading,
		isFetchingNextPage,
		error,
		isLoading,
		fetchNextPage,
		isError,
	} = useGetNFTs(accountAddress);
	const observerElem = useRef<HTMLDivElement | null>(null);
	const { isIntersecting } = useOnScreen(observerElem);
	const isSpinnerVisible = isFetchingNextPage && hasNextPage;

	useEffect(() => {
		if (isIntersecting && hasNextPage && !isFetchingNextPage) {
			fetchNextPage();
		}
	}, [isIntersecting, fetchNextPage, hasNextPage, isFetchingNextPage]);

	useEffect(() => {
		(async () => {
			const hiddenAssets = await get<string[]>(HIDDEN_ASSET_IDS);
			if (hiddenAssets) {
				internalSetHiddenAssetIds(hiddenAssets);
			}
		})();
	}, []);

	const hideAssetId = useCallback(
		async (newAssetId: string) => {
			if (internalHiddenAssetIds.includes(newAssetId)) return;

			const newHiddenAssetIds = [...internalHiddenAssetIds, newAssetId];
			internalSetHiddenAssetIds(newHiddenAssetIds);
			await set(HIDDEN_ASSET_IDS, newHiddenAssetIds);

			const undoHideAsset = async (assetId: string) => {
				try {
					let updatedHiddenAssetIds;
					internalSetHiddenAssetIds((prevIds) => {
						updatedHiddenAssetIds = prevIds.filter((id) => id !== assetId);
						return updatedHiddenAssetIds;
					});
					await set(HIDDEN_ASSET_IDS, updatedHiddenAssetIds);
				} catch (error) {
					// Handle any error that occurred during the unhide process
					toast.error('Failed to unhide asset.');
					// Restore the asset ID back to the hidden asset IDs list
					internalSetHiddenAssetIds([...internalHiddenAssetIds, assetId]);
					await set(HIDDEN_ASSET_IDS, internalHiddenAssetIds);
				}
			};

			const showAssetHiddenToast = async (objectId: string) => {
				toast.custom(
					(t) => (
						<div
							className="flex items-center justify-between gap-2 bg-white w-full shadow-notification border-solid border-gray-45 rounded-full px-3 py-2"
							style={{
								animation: 'fade-in-up 200ms ease-in-out',
							}}
						>
							<div className="flex gap-2 items-center">
								<Check12 className="text-gray-90" />
								<div
									onClick={() => {
										toast.dismiss(t.id);
									}}
								>
									<InlineLink
										to="/nfts/hidden-assets"
										color="hero"
										weight="medium"
										before={
											<Text variant="body" color="gray-80">
												Moved to
											</Text>
										}
										text="Hidden Assets"
										onClick={() => toast.dismiss(t.id)}
									/>
								</div>
							</div>

							<div className="w-auto">
								<InlineLink
									size="bodySmall"
									onClick={() => {
										undoHideAsset(objectId);
										toast.dismiss(t.id);
									}}
									color="hero"
									weight="medium"
									text="UNDO"
								/>
							</div>
						</div>
					),
					{
						duration: 4000,
					},
				);
			};

			showAssetHiddenToast(newAssetId);
		},
		[internalHiddenAssetIds],
	);

	const handleFilterChange = async (tag: any) => {
		await setToSessionStorage<string>('NFTS_PAGE_NAVIGATION', tag.link);
	};
	const { filterType } = useParams();
	const filteredNFTs = useMemo(() => {
		if (!filterType) return ownedAssets?.visual;
		return ownedAssets?.[filterType as DisplayTypes] ?? [];
	}, [ownedAssets, filterType]);

	if (isInitialLoading) {
		return (
			<div className="mt-1 flex w-full justify-center">
				<LoadingSpinner />
			</div>
		);
	}

	const tags = [
		{ name: 'Visual Assets', link: 'nfts' },
		{ name: 'Everything Else', link: 'nfts/other' },
	];

	return (
		<div className="flex flex-1 flex-col flex-nowrap items-center gap-4">
			<PageTitle title="Assets" after={<AssetsOptionsMenu />} />
			<FiltersPortal firstLastMargin tags={tags} callback={handleFilterChange} />

			<Loading loading={isLoading}>
				{isError ? (
					<Alert>
						<div>
							<strong>Sync error (data might be outdated)</strong>
						</div>
						<small>{(error as Error).message}</small>
					</Alert>
				) : null}
				{filteredNFTs?.length ? (
					filterType === DisplayTypes.other ? (
						<NonVisualAssets items={filteredNFTs} />
					) : (
						<VisualAssets items={filteredNFTs} />
					)
				) : (
					<div className="flex flex-1 items-center self-center text-caption font-semibold text-steel-darker">
						No Assets found
					</div>
				)}
			</Loading>
			<div className="mb-5" ref={observerElem}>
				{isSpinnerVisible ? (
					<div className="mt-1 flex w-full justify-center">
						<LoadingSpinner />
					</div>
				) : null}
			</div>
		</div>
	);
}

export default NftsPage;
