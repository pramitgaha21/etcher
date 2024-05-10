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
		<div class="modal-content">
			<h1 class="message-head">{$message.messageTitle}</h1>
			<p class="message">{$message.message}</p>
			<button class="close-button" on:click={closeModal}>Close</button>
		</div>
	</div>
{/if}

<style>
	.message-modal {
		display: flex;
		justify-content: center;
		align-items: center;
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.7); /* Darker semi-transparent background */
		z-index: 1000;
	}

	.modal-content {
		background-color: #fff; /* White background for modal content */
		padding: 2rem;
		border-radius: 0.5rem;
		box-shadow: 0 0 1rem rgba(0, 0, 0, 0.3); /* Shadow effect for depth */
		max-width: 80%;
		max-height: 80%;
		overflow-y: auto; /* Add scrollbar if content overflows */
	}

	.message-head {
		font-size: 2rem;
		font-weight: bold;
		color: #333; /* Darker text color for better visibility */
		margin-bottom: 1rem;
	}

	.message {
		font-size: 1.2rem;
		color: #555; /* Slightly darker text color for better visibility */
		text-align: center;
		margin-bottom: 2rem;
	}

	.close-button {
		background-color: #007bff; /* Unchanged */
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 0.25rem;
		font-size: 1rem;
		cursor: pointer;
	}
</style>

