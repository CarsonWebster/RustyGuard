<script lang="ts">
	import '../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import { AppBar } from '@skeletonlabs/skeleton';
	import { AppRail, AppRailTile, AppRailAnchor } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';

	import { onMount } from 'svelte';
	import { autoModeWatcher } from '@skeletonlabs/skeleton';

	onMount(() => {
		autoModeWatcher();
	});

	import Dashboard from '$lib/pages/dashboard.svelte';
	import Blacklist from '$lib/pages/blacklist.svelte';
	import Settings from '$lib/pages/settings.svelte';
	import Echo from '$lib/pages/echo.svelte';

	import { page } from '$app/stores';

	// const pages = [
	// 	{ page: 'dashboard', component: Dashboard },
	// 	{ page: 'blacklist', component: Blacklist },
	// 	{ page: 'settings', component: Settings }
	// ];
	let currentPage = Echo;
</script>

<AppShell>
	<svelte:fragment slot="header">
		<!-- <AppBar class="">
			<h1 class="h1">
				<span
					class="bg-gradient-to-br from-pink-500 to-violet-500 bg-clip-text text-transparent box-decoration-clone"
				>
					RustyGuard
				</span>
			</h1>
			<button on:click={logPageFunction} type="button" class="btn variant-filled">Log page</button>
		</AppBar> -->
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<AppRail>
			<AppRailTile
				href="/"
				bind:group={currentPage}
				name="Dashboard"
				value={Dashboard}
				title="Dashboard"
			>
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Dashboard</span>
			</AppRailTile>
			<AppRailTile bind:group={currentPage} name="Blacklist" value={Blacklist} title="Blacklist">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Blacklist</span>
			</AppRailTile>
			<AppRailTile bind:group={currentPage} name="Settings" value={Settings} title="Settings">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Settings</span>
			</AppRailTile>
			<AppRailTile bind:group={currentPage} name="Echo" value={Echo} title="Echo">
				<svelte:fragment slot="lead">(icon)</svelte:fragment>
				<span>Echo Test</span>
			</AppRailTile>

			<svelte:fragment slot="trail">
				<AppRailAnchor>
					<LightSwitch />
				</AppRailAnchor>
			</svelte:fragment>
		</AppRail>
	</svelte:fragment>

	<!-- <slot /> -->
	<!-- Render current page component -->
	<svelte:component this={currentPage} />
</AppShell>
