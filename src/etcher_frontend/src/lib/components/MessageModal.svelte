<script lang="ts">
	import { message } from '$lib/stores/message.modal';

	const closeModal = () => {
		message.set({
			show: false,
			messageTitle: '',
			message: ''
		});
	};
</script>

{#if $message.show}
	<div class="message-modal">
		<h1 class="message-head">{$message.messageTitle}</h1>
		<p class="message">{$message.message}</p>
		<button class="close-button" on:click={closeModal}>Close</button>
	</div>
{/if}

<style>
	.message-modal {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 1000;
	}

	.message-head {
		font-size: 2rem;
		font-weight: bold;
		margin-bottom: 1rem;
	}

	.message {
		font-size: 1.2rem;
		text-align: center;
		margin-bottom: 2rem;
	}

	.close-button {
		background-color: #007bff;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 0.25rem;
		font-size: 1rem;
		cursor: pointer;
	}
</style>
