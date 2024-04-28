<script lang="ts">
	import Etcher from '$lib/components/Etcher.svelte';
	import ShowBitcoinBalance from '$lib/components/ShowBitcoinBalance.svelte';
</script>

<main class="page">
	<ShowBitcoinBalance />
	<Etcher />
</main>

<style>
	.page {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		min-height: 100vh;
	}
</style>
