<script lang="ts">
	export let specialTabs: string[]
	export let tasklists: string[]
	export let active: string = specialTabs[0]
</script>

<aside class="menu">
	<p class="menu-label">
		For you
	</p>
	<ul class="menu-list">
		<li>
			{#each specialTabs as tab}
				<a
					class:is-active="{active === tab}"
					on:click="{() => active = tab}"
					>
					{tab}
				</a>
			{/each}
		</li>
	</ul>
	<p class="menu-label">
		Tasklists
	</p>
	<ul class="menu-list">
		{#each tasklists as tasklist}
			<li>
				<a
					class:is-active="{active === tasklist}"
					on:click="{() => active = tasklist}"
					>
					{tasklist}
				</a>
			</li>
		{/each}
	</ul>
</aside>

<style>
	aside {
		position: absolute;
		left: 0;
		top: 0;
		text-align: left;
		padding: 10px;
		width: 200px;
		height: 100vh;
		overflow-y: auto;
		scrollbar-width: none;
	}
</style>
