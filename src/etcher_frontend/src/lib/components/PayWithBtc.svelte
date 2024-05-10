<script lang="ts">
	import { Copy } from '@dfinity/gix-components';
	import ShowBitcoinBalance from './ShowBitcoinBalance.svelte';
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import { onMount } from 'svelte';

	$: btcDepositAddress = '';

	const fetchBtcAddress = async () => {
		const actor = createActor(canisterId);
		let address = await actor.get_deposit_address_for_bitcoin();
		btcDepositAddress = address;
	};

	onMount(async () => {
		await fetchBtcAddress();
	});
</script>

<div class="btc-payment">
	<span>Bitcoin Address: {btcDepositAddress}</span>
	<Copy value={btcDepositAddress} />
	<ShowBitcoinBalance />
</div>

<style>
	.btc-payment {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 20px;
		background-color: #f5f5f5;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.btc-payment span {
		font-size: 16px;
		font-weight: bold;
		color: #333;
		margin-bottom: 10px;
	}
</style>
