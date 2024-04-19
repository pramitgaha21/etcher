<script lang="ts">
	import EtchingModal from '$lib/components/EtchingModal.svelte';
	import { identityStore } from '$lib/stores/auth.store';
</script>

<main>
	<h1 class="title">Etcher</h1>
	<h1 class="welcome-tag">Welcome User {$identityStore?.getPrincipal()}</h1>
	<EtchingModal />
</main>

<style>
	.title {
		font-size: 8rem;
	}
</style>
