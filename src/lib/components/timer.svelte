<script lang="ts">
	import * as timer from '@/lib/stores/timer';
	
	let m: string, s: string;
	$: {
		timer.remainingTime.subscribe((time) => {
			m = Math.floor(time / 60).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false});
			s = Math.floor(time % 60).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false});
		});
	}

	let active: boolean
	$: timer.active.subscribe((a) => active = a);
	let paused: boolean
	$: timer.paused.subscribe((p) => paused = p);

	let width: number
</script>

<div class="timer" bind:clientWidth="{width}">
	<h3 style="font-size: {width / 5}px">
		{m}:{s}
	</h3>

	<div class="buttons">
		{#if !active}
			<button class="button is-primary" on:click="{() => timer.startTimer()}">Start timer</button>
		{:else}
			{#if !paused}
				<button class="button is-warning" on:click="{() => timer.pauseTimer()}">Pause</button>
			{:else}
				<button class="button is-primary" on:click="{() => timer.startTimer()}">Continue</button>
			{/if}
			<button class="button is-danger" on:click="{() => timer.stopTimer()}">Stop</button>
		{/if}
	</div>

</div>
<style>
	.buttons {
		display: inline-block;
	}
</style>
