<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import Card from "../../layout/card.svelte";
  import Item from "./Item.svelte";

  type EventList = {
    items: {
      id: string;
      summary: string;
      start: { dateTime: Date };
    }[];
  };

  let agendaItems: Promise<EventList> = invoke("get_events");
</script>

<Card>
  <h1 class="text-lg underline">Agenda</h1>
  <ul class="list-inside space-y-2">
    {#await agendaItems}
      <p>Loading...</p>
    {:then items}
      {#each items.items as item (item.id)}
        <li class="rounded-md bg-rose-100 drop-shadow-md px-2">
          <Item {item} />
        </li>
      {/each}
    {:catch err}
      <p>{err.msg}</p>
    {/await}
  </ul>
</Card>
