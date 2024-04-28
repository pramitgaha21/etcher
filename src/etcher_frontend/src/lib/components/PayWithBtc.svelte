<script lang="ts">
	import { btcDepositAddress } from '$lib/stores/data.store';
	import { Copy } from '@dfinity/gix-components';
</script>

<div class="btc-payment">
	<span>{$btcDepositAddress}</span>
	<Copy value={$btcDepositAddress} />
</div>

<style>
	.btc-payment {
		margin-top: 1rem;
	}
</style>
