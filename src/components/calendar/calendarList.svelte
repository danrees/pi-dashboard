<script lang="ts">
  import Container from "../../layout/container.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  type CalendarList = {
    items: { summary: string }[];
  };

  let calendarListPromise: Promise<CalendarList> = invoke("get_calendar");
</script>

<Container>
  {#await calendarListPromise}
    <p>Loading</p>
  {:then calendarList}
    <ul>
      {#each calendarList.items as calendar}
        <li>{calendar.summary}</li>
      {/each}
    </ul>
  {:catch err}
    <p>{err.msg}</p>
  {/await}
</Container>
