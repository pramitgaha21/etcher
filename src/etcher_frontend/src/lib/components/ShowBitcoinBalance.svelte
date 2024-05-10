<script lang="ts">
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import { message } from '$lib/stores/message.modal';
	import { onMount } from 'svelte';
	import Button from './ui/button/button.svelte';

	$: btcBalance = 0n;

	const refreshBalance = async () => {
		const actor = createActor(canisterId);
		actor
			.get_btc_balance()
			.then((balance) => {
				console.log(balance);
				btcBalance = balance;
			})
			.catch((e) => {
				message.set({
					show: true,
					messageTitle: 'Failed to get balance',
					message: e
				});
			});
	};

	onMount(async () => {
		await refreshBalance();
	});
</script>

<div class="btc-balance">
	<p>Your Bitcoin Balance in Satoshis: {btcBalance}₿</p>
	<Button on:click={refreshBalance}>Refresh Balance</Button>
</div>

<style>
	.btc-balance {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
		background-color: #f5f5f5;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.btc-balance p {
		font-size: 18px;
		font-weight: bold;
		color: #333;
		margin-right: 20px;
	}
</style>
