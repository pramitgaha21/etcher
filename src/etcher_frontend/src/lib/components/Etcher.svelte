<script lang="ts">
	import { InProd } from '$lib';
	import { createActor, canisterId } from '$lib/declarations/etcher_backend';
	import { type EtchingArgs } from '$lib/declarations/etcher_backend/etcher_backend.did.d';
	import { identityStore } from '$lib/stores/auth.store';
	import { message } from '$lib/stores/message.modal';
	import { HttpAgent } from '@dfinity/agent';
	import PayWithBtc from './PayWithBtc.svelte';
	import PayWithCkBtc from './PayWithCkBTC.svelte';
	import { Button } from './ui/button';
	import { Label } from './ui/label';
	import { Input } from './ui/input';

	let payWithBtc = true;
	let premineFlag = false;

	let rune = '';
	let divisibility: number;
	let symbol: string;
	let premine: bigint;
	let amount: bigint;
	let cap: bigint;
	let height_start: bigint;
	let height_stop: bigint;
	let offset_start: bigint;
	let offset_stop: bigint;
	let turbo: boolean = false;

	$: turboButtonName = turbo ? 'Turbo mode Enabled' : 'Turbo mode Disabled';
	const etchRunestone = async () => {
		console.log('function called');
		let identity = $identityStore;
		if (identity == null) {
			message.set({
				show: true,
				messageTitle: 'Internet Identity not Found',
				message: 'Please Login'
			});
			return;
		}
		const agent = new HttpAgent({ identity });
		if (!InProd) {
			agent.fetchRootKey().catch((e) => {
				message.set({
					show: true,
					messageTitle: 'Failed to Fetch RootKey',
					message: e
				});
				return;
			});
		}
		let symbolIntoNum = symbol.codePointAt(0);
		console.log(symbolIntoNum);
		if (symbolIntoNum == undefined) {
			message.set({
				show: true,
				messageTitle: 'Invalid Symbol',
				message: 'Please Enter a Valid symbol'
			});
			return;
		}
		console.log('constructing arugment');
		let arg: EtchingArgs = {
			rune,
			symbol: symbolIntoNum,
			cap: BigInt(cap),
			amount: BigInt(amount),
			premine: premineFlag ? [BigInt(premine)] : [],
			height_start: BigInt(height_start),
			height_stop: BigInt(height_stop),
			offset_start: BigInt(offset_start),
			offset_stop: BigInt(offset_stop),
			fee_rate: [],
			divisibility: Number(divisibility),
			turbo
		};
		console.log(arg);
		const actor = createActor(canisterId, { agent });
		console.log('calling the function for etching');
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

<div class="etch-rune">
	<div>
		<Button
			on:click={() => {
				payWithBtc = true;
			}}>Pay with Bitcoin</Button
		>
		<Button
			on:click={() => {
				payWithBtc = false;
			}}>Pay with CkBTC</Button
		>
	</div>
	{#if payWithBtc}
		<PayWithBtc />
	{:else}
		<PayWithCkBtc />
	{/if}
	<a href="https://docs.ordinals.com/runes.html" target="_blank"
		>Learn More about the fields: Runes</a
	>
	<div class="form-block">
		<Label>Name</Label>
		<Input bind:value={rune} />
		<Label>Symbol</Label>
		<Input bind:value={symbol} />
		<Label>Divisibility</Label>
		<Input bind:value={divisibility} />
		<Label>Cap</Label>
		<Input type="number" bind:value={cap} />
		<Label>Amount</Label>
		<Input type="number" bind:value={amount} />
		<Label>Height Start</Label>
		<Input type="number" bind:value={height_start} />
		<Label>Height Stop</Label>
		<Input type="number" bind:value={height_stop} />
		<Label>Offset Start</Label>
		<Input type="number" bind:value={offset_start} />
		<Label>Offset Stop</Label>
		<Input type="number" bind:value={offset_stop} />
		<br />
		<div class="premine-block">
			<Button
				on:click={() => {
					premineFlag = !premineFlag;
				}}>Premine</Button
			>
			{#if premineFlag}
				<br />
				<br />
				<Label>Premine</Label>
				<Input type="number" bind:value={premine} />
				<br />
				<br />
			{/if}
		</div>
		<br />
		<br />
		<Button
			on:click={() => {
				turbo = !turbo;
			}}>{turboButtonName}</Button
		>
		<br />
		<br />
		<Button
			on:click={() => {
				etchRunestone();
			}}>Etch Runestone</Button
		>
	</div>
</div>

<style>
	.etch-rune {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		min-height: 100vh;
	}
</style>
