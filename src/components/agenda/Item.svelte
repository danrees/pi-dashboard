<script lang="ts">
  export let item: {
    summary: string;
    start: { dateTime: Date };
  };
</script>

<span>
  {item.summary} -
  <span class="text-slate-500 text-sm">{new Date(item.start.dateTime)}</span>
</span>
