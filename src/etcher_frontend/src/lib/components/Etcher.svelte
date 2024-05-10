<script lang="ts">
	import { message } from '$lib/stores/message.modal';
	import { type EtchingArgs } from '$lib/declarations/etcher_backend/etcher_backend.did';
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import PayWithBtc from './PayWithBtc.svelte';
	import PayWithCkBtc from './PayWithCkBTC.svelte';
	import Button from './ui/button/button.svelte';

	let rune: string;
	let divisibility: number;
	let symbol: string;
	let cap: number;
	let amount: number;
	let percentage: number = 0;
	let startHeight: number;
	let endHeight: number;
	let startOffset: number;
	let endOffset: number;
	let turbo: boolean = true;
	let fee_rate = 20;

	$: turboModeMessage = turbo ? 'Enabled' : 'Disabled';
	$: withBlockHeight = true;
	$: payWithBtc = true;

	const etchRune = async () => {
		console.log('etch rune called');
		const premine = BigInt((percentage * cap) / 100);

		const symbolAsUnicode = symbol.codePointAt(0);
		if (symbolAsUnicode == undefined) {
			message.set({
				show: true,
				messageTitle: 'Invalid Symbol',
				message: 'Please Enter a Valid symbol'
			});
			return;
		}

		let arg: EtchingArgs = {
			rune,
			premine,
			symbol: symbolAsUnicode,
			cap: BigInt(cap),
			amount: BigInt(amount),
			divisibility,
			fee_rate: [BigInt(fee_rate)],
			turbo,
			height: withBlockHeight ? [[BigInt(startHeight), BigInt(endHeight)]] : [],
			offset: !withBlockHeight ? [[BigInt(startOffset), BigInt(endOffset)]] : []
		};

		const actor = createActor(canisterId);
		actor
			.etch_rune(arg)
			.then((result) => {
				console.log(result);
				message.set({
					show: true,
					messageTitle: 'Succesfully submitted the Commit Transaction',
					message: ''
				});
				return;
			})
			.catch((e) => {
				console.log(e);
				message.set({
					show: true,
					messageTitle: 'Failed to Etch Rune',
					message: e
				});
			});
	};
</script>

<form on:submit|preventDefault={etchRune}>
	<label for="payment-mode" class="label-note">
		Pay Fee
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">Pay Fee for Etching with Bitcoin or CkBTC</div>
	</label>
	<div class="term-type">
		<label class="term-type-label" class:selected={payWithBtc}>
			<input
				type="radio"
				name="option"
				value="payWithBtc"
				bind:group={payWithBtc}
				on:change={() => {
					payWithBtc = true;
				}}
			/>
			Pay with Bitcoin
		</label>
		<label class="term-type-label" class:selected={!payWithBtc}>
			<input
				type="radio"
				name="option"
				value="Pay with CkBTC"
				bind:group={payWithBtc}
				on:change={() => {
					payWithBtc = false;
				}}
			/>
			Pay with CkBTC
		</label>
	</div>
	{#if payWithBtc}
		<PayWithBtc />
	{:else}
		<PayWithCkBtc />
	{/if}
	<label for="rune" class="label-note">
		Rune
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">
			Names consist of the letters A through Z and are between one and twenty-six letters long.
		</div>
	</label>
	<input type="text" id="rune" name="rune" bind:value={rune} required />

	<label for="divisibility" class="label-note">
		Divisibility
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">
			A rune's divisibility is how finely it may be divided into its atomic units.
		</div>
	</label>
	<input
		type="number"
		id="divisibility"
		name="divisibility"
		min="0"
		max="38"
		bind:value={divisibility}
		required
	/>

	<label for="symbol" class="label-note">
		Symbol
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">
			A rune's currency symbol is a single Unicode code point, for example $, 😎, or 🧿, displayed
			after quantities of that rune.
		</div>
	</label>
	<input type="text" id="symbol" name="symbol" bind:value={symbol} required />

	<label for="cap" class="label-note">
		Cap
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">
			The number of times a rune may be minted is its cap. A mint is closed once the cap is reached.
		</div>
	</label>
	<input type="number" id="cap" min="0" bind:value={cap} required />

	<label for="amount" class="label-note">
		Amount
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">
			Each mint transaction creates a fixed amount of new units of a rune.
		</div>
	</label>
	<input type="number" id="amount" name="amount" bind:value={amount} required />

	<br />
	<label for="premine" class="label-note">
		Premine
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">
			The etcher of a rune may optionally allocate to themselves units of the rune being etched.
			When set to 100%, Runestone becomes unmintable.
		</div>
	</label>
	<input type="range" id="premine" bind:value={percentage} min="0" max="100" required />
	<span>{percentage}%</span>

	<br />
	<label for="button-separator" class="label-note">
		Set Mint Term with
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">Set mint Term using either Height or Offset</div>
	</label>
	<div class="term-type">
		<label class="term-type-label" class:selected={withBlockHeight}>
			<input
				type="radio"
				name="option"
				value="Using Height"
				bind:group={withBlockHeight}
				on:change={() => {
					withBlockHeight = true;
				}}
			/>
			Using Height
		</label>
		<label class="term-type-label" class:selected={!withBlockHeight}>
			<input
				type="radio"
				name="option"
				value="Using Offset"
				bind:group={withBlockHeight}
				on:change={() => {
					withBlockHeight = false;
				}}
			/>
			Using Offset
		</label>
	</div>
	{#if withBlockHeight}
		<label for="startHeight" class="label-note">
			Start Height
			<span class="note-icon" title="Additional Information">&#9432;</span>
			<div class="note-content">The Mint is Open starting in the block with the given height.</div>
		</label>
		<input type="number" id="startHeight" name="startHeight" bind:value={startHeight} required />

		<label for="endHeight" class="label-note">
			End Height
			<span class="note-icon" title="Additional Information">&#9432;</span>
			<div class="note-content">The Mint is closed in the block with the given height.</div>
		</label>
		<input type="number" id="endHeight" name="endHeight" bind:value={endHeight} required />
	{:else}
		<label for="startOffset" class="label-note">
			Start Offset
			<span class="note-icon" title="Additional Information">&#9432;</span>
			<div class="note-content">
				A mint is open starting in the block whose height is equal to the start offset plus the
				height of the block in which the rune was etched.
			</div>
		</label>
		<input type="number" id="startOffset" name="startOffset" bind:value={startOffset} required />

		<label for="endOffset" class="label-note">
			End Offset
			<span class="note-icon" title="Additional Information">&#9432;</span>
			<div class="note-content">
				A rune may not be minted in or after the block whose height is equal to the end offset plus
				the height of the block in which the rune was etched.
			</div>
		</label>
		<input type="number" id="endOffset" name="endOffset" bind:value={endOffset} required />
	{/if}

	<label for="feeRate" class="label-note">
		Fee Rate
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">More the fee, transaction will be priortized</div>
	</label>

	<input type="range" id="feeRate" name="feeRate" bind:value={fee_rate} min="1" max="200" />
	<p>Fee Rate: {fee_rate}</p>

	<label for="turbo" class="label-note">
		Turbo Mode
		<span class="note-icon" title="Additional Information">&#9432;</span>
		<div class="note-content">Opt in for Future Protocol Changes</div>
	</label>

	<input type="checkbox" bind:checked={turbo} />
	<span>{turboModeMessage}</span>
	<br />

	<Button type="submit">Etch Rune</Button>
</form>

<style>
	/* Reset some default styles */
	* {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
	}

	/* Form styles */
	form {
		max-width: 600px;
		margin: 0 auto;
		padding: 20px;
		background-color: #f5f5f5;
		border-radius: 8px;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
		font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
	}

	/* Label styles */
	.label-note {
		display: block;
		margin-bottom: 5px;
		font-weight: bold;
		color: #333;
	}

	/* Note icon styles */
	.note-icon {
		font-size: 14px;
		color: #666;
		cursor: help;
	}

	/* Note content styles */
	.note-content {
		margin-top: 5px;
		padding: 10px;
		background-color: #e0e0e0;
		border-radius: 4px;
		color: #555;
		font-size: 14px;
		line-height: 1.4;
	}

	/* Input styles */
	input[type='text'],
	input[type='number'] {
		width: 100%;
		padding: 10px;
		border: 1px solid #ccc;
		border-radius: 4px;
		font-size: 16px;
	}

	input[type='range'] {
		width: 100%;
	}

	input[type='checkbox'] {
		appearance: none;
		-webkit-appearance: none;
		-moz-appearance: none;
		width: 20px;
		height: 20px;
		border: 2px solid #ccc;
		border-radius: 4px;
		outline: none;
		cursor: pointer;
		transition: border-color 0.2s ease-in-out;
		position: relative;
	}

	/* Checked State */
	input[type='checkbox']:checked {
		border-color: #4caf50;
		background-color: #4caf50;
	}

	input[type='checkbox']:checked::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 10px;
		height: 10px;
		background-color: #fff;
		border-radius: 50%;
		box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.2);
	}

	/* Hover State */
	input[type='checkbox']:hover {
		border-color: #333;
	}

	/* Focus State */
	input[type='checkbox']:focus {
		box-shadow: 0 0 0 2px rgba(76, 175, 80, 0.4);
	}

	.term-type {
		display: flex;
		flex-direction: row;
		gap: 8px;
	}

	input[type='radio'] {
		appearance: none;
		-webkit-appearance: none;
		-moz-appearance: none;
	}

	input[type='radio']:checked {
		/* Styles for the selected radio button */
		border-color: #98fb98;
		background-color: #98fb98;
	}

	.term-type-label {
		display: block;
		padding: 8px 12px;
		border-radius: 4px;
		background-color: #f0f0f0;
		cursor: pointer;
	}

	.term-type-label.selected {
		background-color: #98fb98;
		color: #333;
	}
</style>
