<script lang="ts">
	import Taskcard from './taskcard.svelte'
	import type { Task } from '@/lib/type'
	let tabs: string[] = ["Task", "Compleated"]
	let active = tabs[0]

	let uid = 1
	let tasks: Task[] = []

	function add(input: any) {
		console.log()
		const task = {
			id: (uid++).toString(),
			compleated: false,
			title: input.value
		};

		tasks = [task, ...tasks]
		input.value = ''
	}

	function remove(task: Task) {
		console.log(typeof(task))
		tasks = tasks.filter(t => t !== task)
	}

	function mark(task: Task) {
		task.compleated = !task.compleated
		remove(task)
		tasks = [task, ...tasks]
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
		<input class="input" type="text" placeholder="Add a new task" on:keydown={e => e.key === 'Enter' && add(e.target)}>
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
