<script lang="ts">
	export let size: number = 1
</script>

<button class="card-header-icon" aria-label="toggle">
	<span class="icon" style="font-size: {size}em">
		<slot></slot>
	</span>
</button>
