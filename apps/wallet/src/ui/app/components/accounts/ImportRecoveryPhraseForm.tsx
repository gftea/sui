// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { yupResolver } from '@hookform/resolvers/yup';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useNavigate } from 'react-router-dom';
import * as Yup from 'yup';
import { mnemonicValidation } from '../../helpers/validation/mnemonicValidation';
import Alert from '../alert';
import { Button } from '_app/shared/ButtonUI';
import { PasswordInput } from '_src/ui/app/shared/forms/controls/PasswordInput';
import { Text } from '_src/ui/app/shared/text';

const MNEMONIC_WORD_COUNT = 12;

const formSchema = Yup.object({
	mnemonic: mnemonicValidation,
});

type FormValues = Yup.InferType<typeof formSchema>;

type ImportRecoveryPhraseFormProps = {
	onSubmit: SubmitHandler<FormValues>;
};

export function ImportRecoveryPhraseForm({ onSubmit }: ImportRecoveryPhraseFormProps) {
	const {
		register,
		formState: { errors, isSubmitting, isValid, touchedFields },
		handleSubmit,
		setValue,
		getValues,
		trigger,
	} = useForm({
		mode: 'all',
		reValidateMode: 'onChange',
		resolver: yupResolver(formSchema),
		defaultValues: {
			mnemonic: Array.from({ length: MNEMONIC_WORD_COUNT }, () => ''),
		},
	});
	const navigate = useNavigate();
	const mnemonic = getValues('mnemonic');

	return (
		<form
			className="flex flex-col justify-between relative h-full"
			onSubmit={handleSubmit(onSubmit)}
		>
			<fieldset className="border-0 m-0 p-0">
				<legend className="pl-2.5">
					<Text variant="pBody" color="steel-darker" weight="semibold">
						Enter your 12-word Recovery Phrase
					</Text>
				</legend>
				<div className="mt-3 grid grid-cols-2 gap-x-2 gap-y-2.5 overflow-auto">
					{mnemonic.map((_, index) => {
						const mnemonicId = `mnemonic.${index}` as const;
						return (
							<label key={index} className="flex flex-col gap-1.5 items-center">
								<Text variant="captionSmall" weight="medium" color="steel-darker">
									{index + 1}
								</Text>
								<PasswordInput
									disabled={isSubmitting}
									onKeyDown={(e) => {
										if (e.key === ' ') {
											e.preventDefault();
											const nextInput = document.getElementsByName(`mnemonic.${index + 1}`)[0];
											nextInput?.focus();
										}
									}}
									onPaste={async (e) => {
										const inputText = e.clipboardData.getData('text');
										const words = inputText
											.trim()
											.split(/\W/)
											.map((aWord) => aWord.trim())
											.filter(String);

										if (words.length > 1) {
											e.preventDefault();
											const pasteIndex = words.length === mnemonic.length ? 0 : index;
											const wordsToPaste = words.slice(0, mnemonic.length - pasteIndex);
											const newMnemonic = [...mnemonic];
											newMnemonic.splice(
												pasteIndex,
												wordsToPaste.length,
												...words.slice(0, mnemonic.length - pasteIndex),
											);
											setValue('mnemonic', newMnemonic);
											trigger('mnemonic');
										}
									}}
									id={mnemonicId}
									{...register(mnemonicId)}
								/>
							</label>
						);
					})}
				</div>
			</fieldset>
			<div className="flex flex-col gap-2.5 pt-3 bg-sui-lightest sticky -bottom-7.5 px-6 pb-7.5 -mx-6 -mb-7.5">
				{touchedFields.mnemonic && errors.mnemonic && <Alert>{errors.mnemonic.message}</Alert>}
				<div className="flex gap-2.5">
					<Button variant="outline" size="tall" text="Cancel" onClick={() => navigate(-1)} />
					<Button
						type="submit"
						disabled={isSubmitting || !isValid}
						variant="primary"
						size="tall"
						loading={isSubmitting}
						text="Add Account"
					/>
				</div>
			</div>
		</form>
	);
}
