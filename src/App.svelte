<script lang="ts">
	import '@/app.scss'
	import { invoke } from '@tauri-apps/api/tauri';
	import Sidebar from './lib/sidebar/sidebar.svelte'
	import Tasklist from './lib/tasklist/tasklist.svelte'

	let specialTabs: string[] = ['Dashboard']
	let tasklists: string[] = []

	let activeTab: string = specialTabs[0]

	rustCalls()
	async function rustCalls() {
		tasklists = await invoke('get_filenames')
	}
</script>
<main>
	<Sidebar {specialTabs} bind:tasklists bind:active={activeTab}/>
	{#if tasklists.includes(activeTab)}
		<Tasklist tasklist={activeTab}/>
	{/if}
</main>
