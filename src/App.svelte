<script lang="ts">
	import '@/app.scss'
	import { invoke } from '@tauri-apps/api/tauri';
	import Sidebar from './lib/sidebar/sidebar.svelte'
	import Tasklist from './lib/tasklist/tasklist.svelte'

	let specialTabs: string[] = ['Dashboard', 'Important']
	let tasklists: string[] = []

	let activeTab: string
	rustCalls()

	async function rustCalls() {
		tasklists = await invoke('get_filenames')
	}
</script>
<main>
	<Sidebar bind:active={activeTab} {specialTabs} {tasklists}/>
	{#if tasklists.includes(activeTab)}
		<Tasklist tasklist={activeTab}/>
	{/if}
</main>
