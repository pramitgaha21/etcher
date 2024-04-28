<script lang="ts">
	import { InProd } from '$lib';
	import { canisterId, createActor } from '$lib/declarations/etcher_backend';
	import { identityStore } from '$lib/stores/auth.store';
	import { blockId, ckbtcDepositAddress } from '$lib/stores/data.store';
	import { message } from '$lib/stores/message.modal';
	import { HttpAgent } from '@dfinity/agent';
	import { Copy, QRCode } from '@dfinity/gix-components';
	import Button from './ui/button/button.svelte';

	$: paymentStatus = '';

	$: {
		console.log($ckbtcDepositAddress);
	}

	const confirmAndConvertCkbtc = async () => {
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
					messageTitle: 'Failed to fetch RootKey',
					message: e
				});
				return;
			});
		}
		const actor = createActor(canisterId, { agent });
		actor
			.confirm_and_convert_ckbtc()
			.then((id) => {
				console.log(id);
				return;
			})
			.catch((e) => {
				message.set({
					show: true,
					message: e,
					messageTitle: 'Failed to confirm and convert CkBTC'
				});
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
			.query_converstion_status(id)
			.then((result) => {
				console.log(result);
			})
			.catch((e) => {
				console.log(e);
			});
	};
</script>

<div class="ckbtc-payment">
	<div class="address">
		<span>{$ckbtcDepositAddress}</span>
		<Copy value={$ckbtcDepositAddress} />
	</div>
	<div class="convert-button">
		<Button on:click={confirmAndConvertCkbtc}>Convert CkBTC</Button>
	</div>
	<div class="status">
		<span>Status: {'hi'}</span>
		<Button on:click={queryTransactionStatus}>Refresh Status</Button>
	</div>
</div>

<style>
	.ckbtc-payment {
		margin-top: 1rem;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.address {
		margin: 0.3rem;
	}

	.convert-button {
		margin: 0.3rem;
	}

	.status {
		margin: 0.3rem;
	}
</style>
