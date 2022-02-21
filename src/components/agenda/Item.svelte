<script lang="ts">
  export let item: { summary: string; created: Date };
</script>

<div class="flex coloumns-2">
  <span>{item.summary}</span>
  <span>{new Date(item.created)}</span>
</div>
