<template>
	<main>
		<h1>La liste des taches</h1>
		<ul>
			<li v-for="task in tasks">
				{{ task.title }}
			</li>
		</ul>
	</main>
</template>

<script lang="ts">
import { useTasksStore } from "@/stores/tasks";
import { mapActions, mapState } from "pinia";

export default {
	mounted() {
		if (this.tasks.length === 0) {
			this.fetchTasks();
		}
	},
	methods: {
		...mapActions(useTasksStore, ["fetchTasks"]),
	},
	computed: {
		...mapState(useTasksStore, ['tasks'])
	}
};
</script>
