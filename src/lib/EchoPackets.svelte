<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let interfaceNames: String[] = [];
	let selectedInterface = '';

	onMount(() => {
		getInterfaces();
	});

	async function getInterfaces() {
		try {
			interfaceNames = await invoke('get_network_interfaces');
			console.log(interfaceNames);
		} catch (error) {
			console.error('Failed to get network interfaces', error);
		}
	}
</script>

<div class="card p-4 flex gap-4 flex-col">
	<label class="label">
		<span>Select a network interface</span>
		<select class="select" bind:value={selectedInterface}>
			{#each interfaceNames as interfaceName}
				<option value={interfaceName}>{interfaceName}</option>
			{/each}
		</select>
	</label>
	<p class="card-footer">Selected interface: {selectedInterface}</p>
</div>
