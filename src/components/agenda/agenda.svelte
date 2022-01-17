<script lang="ts">
  import Card from "../../layout/card.svelte";
  import Item from "./Item.svelte";

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

<Card>
  <ul class="list-disc list-inside">
    {#each agendaItems as item (item.id)}
      <li>
        <Item item={item.value} />
      </li>
    {/each}
  </ul>
</Card>
