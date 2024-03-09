<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';

	let interfaceNames: string[] = [];
	let selectedInterface = '';
	let packets: string[] = [];

	async function getInterfaces() {
		try {
			interfaceNames = await invoke('get_network_interfaces');
			// console.log(interfaceNames);
		} catch (error) {
			alert('Failed to get network interfaces: ' + error);
		}
	}

	async function startCapture() {
		try {
			await invoke('log_packets', { interfaceName: selectedInterface });
		} catch (error) {
			alert('Failed to start log_packets: ' + error);
		}
	}

	onMount(async () => {
		await getInterfaces();
		const unlistenPackets = await listen('packet', (event) => {
			packets = [...packets, event.payload as string];
		});
	});
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

	<button class="btn" on:click={startCapture}>Start capture</button>

	<div class="card p-4">
		<h2 class="h2">Packets</h2>
		<ul class="list">
			{#each packets as packet}
				<li>{packet}</li>
			{/each}
		</ul>
	</div>
</div>
