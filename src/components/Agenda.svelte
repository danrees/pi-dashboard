<script lang="ts">
  import Item from "./agenda/Item.svelte";

  let agendaItems = [
    {
      id: 1,
      value: "Shopping",
    },
    {
      id: 2,
      value: "Get gas",
    },
  ];
</script>

<div>
  <ul class="list-disc list-outside">
    {#each agendaItems as item (item.id)}
      <li class="bg-blue-500">
        <Item item={item.value} />
      </li>
    {/each}
  </ul>
</div>
