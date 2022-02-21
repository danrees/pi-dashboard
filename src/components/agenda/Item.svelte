<script lang="ts">
  export let item: {
    summary: string;
    created: Date;
    start: { dateTime: Date };
  };
</script>

<div class="flex coloumns-2">
  <p>{item.summary}-{new Date(item.start.dateTime)}</p>
</div>
