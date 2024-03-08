<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';

	let countData: number[] = [];

	// let interfaces: any = [];
	// let selectedInterface = '';
	// let isSniffing: boolean = false;
	// let packetData: any[] = [];

	onMount(async () => {
		let unlistenCount = await listen('count', (event) => {
			console.log('count event: ', event.payload);
			countData = [...countData, (parseInt(event.payload as string))];
		});
		// refreshInterfaces();
		// setupListeners();
	});

	async function startCounting() {
		try {
			await invoke('start_counting');
		}
		catch (error) {
			alert('Failed to start counting: ' + error);
		}
	}

	// async function refreshInterfaces() {
	// 	try {
	// 		interfaces = await invoke('get_network_interfaces');
	// 		// if (interfaces.length > 0) {
	// 		// 	selectedInterface = interfaces[0];
	// 		// }
	// 	} catch (error) {
	// 		console.error('Failed to get network interfaces', error);
	// 	}
	// }

	// async function setupListeners() {
	// 	await listen('packet', (event) => {
	// 		packetData = [...packetData, event.payload];
	// 	});
	// 	await listen('packet_error', (event) => {
	// 		alert('Error: ' + event.payload);
	// 		stopSniffing();
	// 	});
	// }

	// async function startSniffing() {
	// 	if (!selectedInterface) {
	// 		alert('Please select an interface to start sniffing.');
	// 		return;
	// 	}
	// 	isSniffing = true;
	// 	packetData = [];
	// 	try {
	// 		await invoke('start_packet_sniffing', { interfaceName: selectedInterface });
	// 	} catch (error) {
	// 		alert('Failed to start sniffing: ' + error);
	// 		isSniffing = false;
	// 	}
	// }

	// async function stopSniffing() {
	// 	isSniffing = false;
	// 	await invoke('stop_packet_sniffing');
	// }

	// onDestroy(() => {
	// 	stopSniffing();
	// });
</script>

<div class="container h-full mx-auto flex justify-center">
	<div class="space-y-5">
		<h1 class="h1">Echo Testing Page️</h1>

		<button class="btn" on:click={startCounting}>Start Counting</button>
		{#if countData.length > 0}
			<div>
				<h2>Count Data:</h2>
				{#each countData as data}
					<p>{data}</p>
				{/each}
			</div>
		{/if}



		<!-- <select class="select" bind:value={selectedInterface}>
			{#each interfaces as face}
				<option value={face}>{face}</option>
			{/each}
		</select>

		<button class="btn" on:click={refreshInterfaces}>Refresh Interfaces</button>
		{#if isSniffing}
			<button class="btn" on:click={stopSniffing}>Stop Sniffing</button>
		{:else}
			<button class="btn" on:click={startSniffing}>Start Sniffing on [{selectedInterface}]</button>
		{/if}

		{#if packetData.length > 0}
			<div>
				<h2>Packet Data:</h2>
				{#each packetData as data}
					<p>{data}</p>
				{/each}
			</div>
		{/if} -->
	</div>
</div>
