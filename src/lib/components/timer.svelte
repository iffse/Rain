<script lang="ts">
	import * as timer from '@/lib/stores/timer'
	import Icon from '@/lib/slots/icon.svelte'
	
	let m: string, s: string;
	$: {
		timer.remainingTime.subscribe((time) => {
			m = Math.floor(time / 60).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false})
			s = Math.floor(time % 60).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false})
		})
	}

	let active: boolean
	$: timer.active.subscribe((a) => active = a)
	let paused: boolean
	$: timer.paused.subscribe((p) => paused = p)
	let compleatedCount: number
	$: timer.compleatedCount.subscribe((c) => compleatedCount = c)

	let width: number

	$: iconSize = width/300 > 2 ? width/300 : 2
</script>

<div class="timer" bind:clientWidth="{width}">
	<h3 style="font-size: {width / 75}em">
		{m}:{s}
	</h3>

	<div class="buttons">
		{#if !active}
			<Icon size={iconSize}>
				<i class="fa-regular fa-play" aria-hidden="true" on:click="{() => timer.startTimer()}"></i>
			</Icon>
		{:else}
			<Icon size={iconSize}>
				<i class="fa-regular fa-stop" aria-hidden="true" on:click="{() => timer.stopTimer()}"></i>
			</Icon>
			{#if !paused}
				<Icon size={iconSize}>
					<i class="fa-regular fa-pause" aria-hidden="true" on:click="{() => timer.pauseTimer()}"></i>
				</Icon>
			{:else}
				<Icon size={iconSize}>
					<i class="fa-regular fa-play" aria-hidden="true" on:click="{() => timer.startTimer()}"></i>
				</Icon>
			{/if}
			<Icon size={iconSize}>
				<i class="fa-regular fa-forward" aria-hidden="true" on:click="{() => timer.skipTimer()}"></i>
			</Icon>
		{/if}
	</div>

	{#if compleatedCount > 0}
	{compleatedCount} {compleatedCount < 2 ? 'round' : 'rounds'} compleated!
	{/if}
</div>

<style>
	.timer {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}
</style>
