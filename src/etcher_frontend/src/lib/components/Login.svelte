<script lang="ts">
	import { InProd } from '$lib';
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import { connectII, identityStore } from '$lib/stores/auth.store';
	import { btcDepositAddress, ckbtcDepositAddress } from '$lib/stores/data.store';
	import { message } from '$lib/stores/message.modal';
	import { HttpAgent } from '@dfinity/agent';
	import Button from './ui/button/button.svelte';

	$: buttonName = $identityStore == null ? 'Connect' : 'Connected';

	const handleClick = async () => {
		let result = await connectII();
		console.log(result);
		console.log('now handling other parts');
		let identity = $identityStore;
		if (identity == null) {
			message.set({
				show: true,
				messageTitle: 'Internet identity not Found',
				message: 'Internet Identity not Found, Please Login'
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
				return;
			});
		}
		const actor = createActor(canisterId, { agent });
		actor
			.get_deposit_address_for_bitcoin()
			.then((address) => {
				console.log(address);
				btcDepositAddress.set(address);
			})
			.catch((e) => {
				message.set({
					show: true,
					messageTitle: 'Failed to fetch address',
					message: e
				});
				return;
			});
		actor
			.get_deposit_address_for_ckbtc()
			.then((address) => {
				console.log(address);
				ckbtcDepositAddress.set(address);
			})
			.catch((e) => {
				message.set({
					show: true,
					messageTitle: 'Failed to fetch address',
					message: e
				});
				return;
			});
	};
</script>

<Button on:click={handleClick}>{buttonName}</Button>
