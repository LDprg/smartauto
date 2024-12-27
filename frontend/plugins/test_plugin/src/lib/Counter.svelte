<script lang="ts">
	let count: number = $state(0);
	const increment = () => {
		count += 1;
	};
</script>

<button onclick={increment} class="rounded-full bg-blue-100 px-5 py-3 hover:bg-blue-400">
	count is {count}
</button>
