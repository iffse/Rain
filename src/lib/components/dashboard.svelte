<script lang="ts">
	import Timer from './timer.svelte'
</script>

<div class="columns">
	<div class="column task-column">
		<h2 class="title is-5">To do</h2>
	</div>
	<div class="column time-column">
		<div class="timer">
			<Timer />
		</div>
	</div>
</div>

<style>
.columns {
	margin: 0;
	height: 100%;
}
.title {
	text-align: left;
}
.time-column {
	margin: auto;
	place-items: center;
}
</style>
