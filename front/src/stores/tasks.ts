import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

interface TasksState {
	_tasks: Task[],
}

export const useTasksStore = defineStore('tasks', {
	state: (): TasksState => {
		return {
			_tasks: [],
		}
	},
	getters: {
		tasks(state): Task[] { return state._tasks },
	},
	actions: {
		async fetchTasks() {
			this._tasks = await (await fetch('http://127.0.0.1:3000/api/tasks')).json();
		}
	}
})

interface Task {
	title: string;
	content: string;
	status: string;
}