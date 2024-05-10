<script lang="ts">
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import { blockId } from '$lib/stores/data.store';
	import { message } from '$lib/stores/message.modal';
	import { Copy } from '@dfinity/gix-components';
	import Button from './ui/button/button.svelte';
	import { onMount } from 'svelte';
	import ShowBitcoinBalance from './ShowBitcoinBalance.svelte';

	$: ckbtcDepositAddress = '';
	$: paymentStatus = 'Unknown';
	$: console.log('block Id', $blockId);

	const fetchCkbtcAddress = async () => {
		const actor = createActor(canisterId);
		let address = await actor.get_deposit_address_for_ckbtc();
		ckbtcDepositAddress = address;
	};

	const confirmAndConvertCkbtc = async () => {
		const actor = createActor(canisterId);
		actor
			.confirm_and_convert_ckbtc()
			.then((id) => {
				console.log(id);
				blockId.set(id);
				return queryTransactionStatus();
			})
			.catch((e) => {
				message.set({
					show: true,
					message: e,
					messageTitle: 'Failed to confirm and convert CkBTC'
				});
				return;
			});
	};

	const queryTransactionStatus = async () => {
		let id = $blockId;
		if (id == null) {
			message.set({
				show: true,
				messageTitle: 'No Block Id Found',
				message: 'No CkBTC conversion Transaction Found'
			});
			return;
		}
		const etcher_backend = createActor(canisterId);
		etcher_backend
			.query_conversion_status(id)
			.then((status) => {
				paymentStatus = status;
				return;
			})
			.catch((e) => {
				message.set({
					show: true,
					messageTitle: 'Failed to Fetch Status',
					message: e
				});
			});
	};

	onMount(async () => {
		console.log('called');
		await fetchCkbtcAddress();
	});
</script>

<div class="ckbtc-payment">
	<div class="address">
		<span>CkBTC Address: {ckbtcDepositAddress}</span>
		<Copy value={ckbtcDepositAddress} />
	</div>
	<div class="convert-button">
		<Button on:click={confirmAndConvertCkbtc}>Confirm Deposit of CkBTC</Button>
	</div>
	{#if $blockId !== null}
		<div class="status">
			<p class="refresh-alert">
				Don't Refresh or Close the page, The block Id will be lost &#9432;
			</p>
			<p class="status-message">Status: {paymentStatus}</p>
			<Button on:click={queryTransactionStatus}>Refresh Status</Button>
		</div>
	{/if}
	<ShowBitcoinBalance />
</div>

<style>
	.ckbtc-payment {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 20px;
		background-color: #f5f5f5;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.ckbtc-payment span {
		font-size: 16px;
		font-weight: bold;
		color: #333;
		margin-bottom: 10px;
	}

	.address {
		margin: 0.3rem;
	}

	.convert-button {
		margin: 0.3re;
	}

	.status {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1rem;
		background-color: #f5f5f5;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.refresh-alert {
		padding: 10px;
		background-color: #fff3cd;
		color: #856404;
		border: 1px solid #ffeeba;
		border-radius: 4px;
		font-weight: bold;
		max-width: 100%;
		overflow-wrap: break-word;
		word-wrap: break-word;
		hyphens: auto;
	}

	.status-message {
		font-size: 16px;
		font-weight: bold;
		color: #333;
		padding: 10px;
		background-color: #f5f5f5;
		border: 1px solid #ddd;
		border-radius: 4px;
		max-width: 100%;
		overflow-wrap: break-word;
		word-wrap: break-word;
		hyphens: auto;
	}
</style>
