<script lang="ts">
	import { identityStore } from '$lib/stores/auth.store';
	import { btcDepositAddress, ckbtcDepositAddress } from '$lib/stores/data.store';
	import { message } from '$lib/stores/message.modal';
	import { HttpAgent } from '@dfinity/agent';
	import { QRCode } from '@dfinity/gix-components';

	let showModal = false;
	let showPayment = false;
	let payWithBitcoin = true;

	const toggleModal = () => {
		showModal = !showModal;
	};

	const payWithBtc = () => {
		payWithBitcoin = true;
	};
	const paywithCkBTC = () => {
		payWithBitcoin = false;
	};

	const fetchRetrievalStatus = async () => {
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
	};

	const etchRune = async () => {
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
		// TODO
	};
</script>

{#if !showModal}
	<div class="btn">
		<button on:click={toggleModal}>Etch Rune</button>
	</div>
{:else}
	<modal>
		<div class="etch-form"></div>
		<div class="payment-block">
			<button
				class="payment-button"
				on:click={() => {
					showPayment = true;
					payWithBtc();
				}}>Pay with Bitcoin</button
			>
			<button
				class="payment-button"
				on:click={() => {
					showPayment = true;
					paywithCkBTC();
				}}>Pay with ckBTC</button
			>
			{#if showPayment}
				{#if payWithBitcoin}
					<h1>Paying with bitcoin</h1>
					<QRCode value={$btcDepositAddress} ariaLabel="Bitcoin Deposit Address" />
				{:else}
					<h1>Paying with ckBTC</h1>
					<QRCode value={$ckbtcDepositAddress} ariaLabel="ckBTC Deposit Address" />
					<button>Confirm Deposit</button>
					<p class="payment-status">Payment Status</p>
				{/if}
			{/if}
		</div>
		<div class="end">
			<button class="cancel-button" on:click={toggleModal}>Cancel</button>
			<button class="etch-rune-button" on:click={etchRune}>Etch Rune</button>
		</div>
	</modal>
{/if}

<style>
</style>
