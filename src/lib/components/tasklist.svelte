<script lang="ts">
	import Taskcard from './taskcard.svelte'
	import { invoke } from '@tauri-apps/api'
	import type { Task } from '@/lib/shared/type'
	import { onDestroy } from 'svelte';
	export let tasklist: string
	const tabs: string[] = ["Tasks", "Done"]
	let active = tabs[0]

	let tasks: Task[] = []

	let saved = true
	let saveTimes = -1
	let saveTimeout: string|number|NodeJS.Timeout
	let list = tasklist

	$: tasks, startSaveTimer()
	$: tasklist, changeTaskList()

	onDestroy(() => {
		if (!saved) {
			clearTimeout(saveTimeout)
			writeList()
		}
	})

	function startSaveTimer() {
		if (saveTimes < 1) {
			saveTimes += 1
			return
		}
		saved = false
		clearTimeout(saveTimeout),
		saveTimeout = setTimeout(writeList, 30e3)
	}
	async function changeTaskList() {
		if (!saved) {
			clearTimeout(saveTimeout)
			writeList()
		}
		try {
			tasks = JSON.parse(await invoke('load_file', {name: tasklist}))
		} catch (e) {
			tasks = []
		}
		saved = true
		saveTimes = 0
		list = tasklist
	}
	async function addTask(input: any) {
		const task: Task = await invoke('new_task', {title: input.value})

		tasks = [task, ...tasks]
		input.value = ''
	}
	function remove(task: Task) {
		tasks = tasks.filter(t => t !== task)
	}
	function mark(task: Task) {
		task.compleated = !task.compleated
		remove(task)
		tasks = [task, ...tasks]
	}
	function writeList() {
		invoke('write_file', {name: list, content: JSON.stringify(tasks)})
		saved = true
	}
</script>

<div class="tasklist">
	<div class="tabs">
		<ul>
			{#each tabs as tab}
				<li
					class:is-active="{active === tab}"
					on:click="{() => active = tab}"
					>
					<a>{tab}</a>
				</li>
			{/each}
		</ul>
	</div>

	{#if active == tabs[0]}
		<div class="tasks">
			{#each tasks.filter(t => !t.compleated) as task}
				<Taskcard {task} on:mark="{() => mark(task)}" />
			{/each}
		</div>
		<input class="input" type="text" placeholder="Add a new task" style="bottom: 10px;"
			on:keydown={e => e.key === 'Enter' && addTask(e.target)}>
	{:else}
		<div class="tasks">
			{#each tasks.filter(t => t.compleated) as task}
				<Taskcard {task} on:mark="{() => mark(task)}" />
			{/each}
		</div>
	{/if}
</div>

<style>
	.tasklist {
		height: 100%;
		display: flex;
		flex-direction: column;
	}
	.tasks {
		overflow-y: scroll;
		height: 100%;
	}
</style>
