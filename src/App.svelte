<script lang="ts">
	import '@/app.scss'
	import { invoke } from '@tauri-apps/api/tauri';
	import Sidebar from './lib/components/sidebar.svelte'
	import Tasklist from './lib/components/tasklist.svelte'
	import Dashboard from './lib/components/dashboard.svelte'

	const specialTabs: string[] = ['Dashboard']
	let tasklists: string[] = []

	let activeTab: string = specialTabs[0]

	rustCalls()
	async function rustCalls() {
		tasklists = await invoke('get_filenames')
	}
</script>
<main>
	<Sidebar {specialTabs} bind:tasklists bind:active={activeTab}/>
	<div class="tab-content">
		{#if activeTab == specialTabs[0]}
			<Dashboard/>
		{/if}
		{#if tasklists.includes(activeTab)}
			<Tasklist tasklist={activeTab}/>
		{/if}
	</div>
</main>

<style>
	main {
		height: 100%;
		overflow-y: scroll;
	}
	.tab-content {
		margin-left: 200px;
		height: 100%;
		overflow-y: scroll;
	}
</style>
