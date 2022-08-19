<script lang="ts">
	export let specialTabs: string[]
	export let tasklists: string[]
	export let active: string = specialTabs[0]
	
	import { clickOutSide } from '@/lib/events'
	let newInput = false

	function addList(input: any) {
		let name: string = input.value.replace(/[^A-zÀ-ž0-9:.]/g, '')
		input.value = ''
		newInput = false

		if (name == '' || tasklists.includes(name)) { return }
		tasklists = [...tasklists, name]
	}
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
		{#if !newInput}
		<button class="button is-text" aria-label="new list" on:click={() => {newInput = true}}>
			<span class="icon-text">
				<span class="icon">
					<i class="fas fa-plus"></i>
				</span>
				<span>New</span>
			</span>
		</button>
		{:else}
		<input
			class="input" type="text" placeholder="Add a new task"
			on:keydown="{e => e.key === 'Enter' && addList(e.target)}"
			use:clickOutSide on:outclick="{() => (newInput = false)}"
			>
		{/if}
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
		height: 100%;
		overflow-y: scroll;
		scrollbar-width: 0;
	}
	button {
		margin-left: 5px;
	}
</style>
