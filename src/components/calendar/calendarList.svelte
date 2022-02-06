<script lang="ts">
  import Container from "../../layout/container.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  type CalendarList = {
    items: { summary: string; id: string }[];
  };

  let calendarListPromise: Promise<CalendarList> = invoke("get_calendar");

  let chosenCalendar: string;
  let config: { calendar_id: string };
  invoke("load_config").then((data: { calendar_id: string }) => {
    config = data;
    chosenCalendar = data.calendar_id;
  });

  const handleSubmit = async () => {
    try {
      config = await invoke("save_config", { calendarId: chosenCalendar });
    } catch (err) {
      alert("Unable to save calendar id to config: " + err);
    }
  };
</script>

<Container>
  {#await calendarListPromise}
    <p>Loading</p>
  {:then calendarList}
    <form on:submit|preventDefault={handleSubmit}>
      <select bind:value={chosenCalendar}>
        {#each calendarList.items as calendar}
          <option value={calendar.id}>{calendar.summary}</option>
        {/each}
      </select>
      <button type="submit">Select</button>
    </form>
  {:catch err}
    <p>{err.msg}</p>
  {/await}
</Container>
