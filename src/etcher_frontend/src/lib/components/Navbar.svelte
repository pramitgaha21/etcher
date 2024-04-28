<script lang="ts">
	import Login from './Login.svelte';
</script>

<nav>
	<div class="navbar-center">
		<h1>Etcher</h1>
	</div>
	<div class="navbar-right">
		<Login />
	</div>
</nav>

<style>
	nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background-color: #fff;
		color: #333;
		padding: 10px 20px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
	}

	.navbar-center h1 {
		margin: 0;
		font-size: 24px;
	}

	.navbar-right {
		color: #fff;
		border: none;
		padding: 8px 16px;
		font-size: 16px;
		cursor: pointer;
		border-radius: 4px;
	}
</style>
