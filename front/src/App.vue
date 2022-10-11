<template>
	<div class="dashboard">
		<aside class="dashboard-menu">
			<nav>
				<RouterLink :to="{name: 'home'}" class="dashboard-menu-link">Accueil</RouterLink>
				<RouterLink :to="{name: 'tasks'}" class="dashboard-menu-link">About</RouterLink>
			</nav>
		</aside>

		<div class="dashboard-content">
			<RouterView />
		</div>
	</div>
</template>

<script lang="ts">
import { RouterLink, RouterView } from 'vue-router';

</script>

<style lang="scss" scoped>
.dashboard {
	display: grid;
	grid-template-columns: minmax(250px, 350px) 1fr;
	height: 100%;
	padding: 1rem;
	gap: 1rem;

	.dashboard-menu,
	.dashboard-content {
		box-shadow: 0 0 10px 2px #111;
		background-color: var(--color-background-mute);
		border-radius: 1rem;
		overflow-x: hidden;
	}

	.dashboard-menu {
		overflow: hidden;

		.dashboard-menu-link {
			height: 50px;
			text-decoration: none;
			color: var(--color-text);
			padding: 0.5rem 1rem;
			display: flex;
			align-items: center;

			&.router-link-active,
			&:hover {
				background-color: var(--color-background);
			}
		}
	}

	.dashboard-content {
		overflow-y: auto;
		padding: 1rem;
	}
}
</style>

<style>
#app {
	height: 100vh;
}
</style>