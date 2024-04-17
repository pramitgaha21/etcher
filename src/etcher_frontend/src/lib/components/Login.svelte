<script lang="ts">
	import { connectII, identityStore } from '$lib/stores/auth.store';
	import { message } from '$lib/stores/message.modal';
	import { HttpAgent } from '@dfinity/agent';

	$: buttonName = $identityStore == null ? 'Connect' : 'Connected';

	const handleClick = async () => {
		await connectII();
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
</script>

<button class="btn" on:click={handleClick}>{buttonName}</button>

<style>
	.btn {
		display: inline-block;
		font-weight: 600;
		color: #fff;
		text-align: center;
		text-decoration: none;
		vertical-align: middle;
		cursor: pointer;
		user-select: none;
		background-color: #007bff;
		border: 1px solid #007bff;
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		line-height: 1.5;
		border-radius: 0.25rem;
	}
</style>
