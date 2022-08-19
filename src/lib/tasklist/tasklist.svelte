<script lang="ts">
	import Taskcard from './taskcard.svelte'
	import { invoke } from '@tauri-apps/api'
	import type { Task } from '@/lib/type'
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
		console.log("timer")
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
		let lists = await invoke('load_file', {name: tasklist})

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
		console.log("Saved " + list)
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
		<input class="input" type="text" placeholder="Add a new task" on:keydown={e => e.key === 'Enter' && addTask(e.target)}>
		{#each tasks.filter(t => !t.compleated) as task}
			<Taskcard {task} on:mark="{() => mark(task)}" />
		{/each}
	{:else}
		{#each tasks.filter(t => t.compleated) as task}
			<Taskcard {task} on:mark="{() => mark(task)}" />
		{/each}
	{/if}
</div>

<style>
	.tasklist {
		margin-left: 180px;
	}
</style>
