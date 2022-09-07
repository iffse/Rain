<script lang="ts">
	import * as timer from '@/lib/stores/timer';
	
	let m: string, s: string;
	$: {
		timer.remainingTime.subscribe((time) => {
			m = Math.floor(time / 60).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false});
			s = (time % 60).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false});
		});
	}

	let active: boolean
	$: timer.active.subscribe((a) => active = a);
	let paused: boolean
	$: timer.paused.subscribe((p) => paused = p);
</script>

<h3 class="title is-1">
	{m}:{s}
</h3>

<div class="buttons">
	{#if !active}
		<button class="button is-primary" on:click="{() => timer.startTimer()}">Start timer</button>
	{:else}
		{#if !paused}
			<button class="button is-warning">Pause</button>
		{:else}
			<button class="button is-primary">Continue</button>
		{/if}
		<button class="button is-danger" on:click="{() => timer.stopTimer()}">Stop</button>
	{/if}
</div>

<style>
	.buttons {
		display: inline-block;
	}
</style>
