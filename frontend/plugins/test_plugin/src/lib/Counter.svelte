<script lang="ts">
  let count: number = $state(0)
  const increment = () => {
    count += 1
  }
</script>

<button onclick={increment} class="bg-blue-100 hover:bg-blue-400 rounded-full px-5 py-3">
  count is {count}
</button>
