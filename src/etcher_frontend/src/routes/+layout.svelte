<script lang="ts">
	import '../app.pcss';
	import MessageModal from '$lib/components/MessageModal.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
</script>

<Navbar />
<MessageModal />
<slot></slot>
