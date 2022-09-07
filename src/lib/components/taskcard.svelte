<script lang="ts">
	import type { Task } from '@/lib/type'
	export let task: Task

	import { createEventDispatcher } from 'svelte'
	const dispatch = createEventDispatcher()
	function mark() {
		dispatch('mark')
	}
</script>

<div class="card">
	<header class="card-header">
		<button class="card-header-icon" aria-label="toggle" on:click="{mark}">
			<span class="icon">
				{#if task.compleated == false}
					<i class="fa-regular fa-circle"aria-hidden="true"></i>
				{:else}
					<i class="fa-regular fa-circle-check compleated" aria-hidden="true"></i>
				{/if}
			</span>
		</button>
		<p class="card-header-title">
			{task.title}
		</p>
		<button class="card-header-icon" aria-label="more">
			<span class="icon">
				<i class="fas fa-angle-down" aria-hidden="true"></i>
			</span>
		</button>
	</header>
</div>

<style>
	.card {
		margin: 5px 0;
	}
</style>
