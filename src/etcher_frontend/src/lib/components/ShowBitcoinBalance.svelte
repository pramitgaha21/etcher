<script lang="ts">
	import { InProd } from '$lib';
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import { identityStore } from '$lib/stores/auth.store';
	import { message } from '$lib/stores/message.modal';
	import { HttpAgent } from '@dfinity/agent';
	import Button from './ui/button/button.svelte';

	$: btcBalance = BigInt(0);

	const refreshBalance = async () => {
		let identity = $identityStore;
		if (identity == null) {
			message.set({
				show: true,
				messageTitle: 'Internet Identity not Logged in',
				message: 'Please Login with your Internet Identity'
			});
			return;
		}
		const agent = new HttpAgent({ identity });
		if (!InProd) {
			await agent.fetchRootKey().catch((e) => {
				message.set({
					show: true,
					messageTitle: 'Failed to fetch root key',
					message: e
				});
			});
		}
		const actor = createActor(canisterId, { agent });
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
</script>

<div class="btc-balance">
	<p>Your Bitcoin Balance: {btcBalance / BigInt(8)}</p>
	<Button on:click={refreshBalance}>Refresh Balance</Button>
</div>

<style>
	.btc-balance {
		display: flex;
		align-items: center;
	}

	.btc-balance p {
		margin-right: 1rem;
	}
</style>
