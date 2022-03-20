<script lang="ts">
  import { Meta, Story, Template } from "@storybook/addon-svelte-csf";
  import { mockIPC } from "@tauri-apps/api/mocks";
  import CalendarList from "./calendarList.svelte";

  mockIPC((cmd, args) => {
    switch (cmd) {
      case "load_config":
        const lc = new Promise((resolve) => {
          resolve({
            data: { calendar_id: "1" },
          });
        });
        return lc;
      case "get_calendar":
        const gc = new Promise((resolve) => {
          resolve({
            items: [
              { summary: "calendar1", id: "1" },
              { summary: "calendar2", id: "2" },
            ],
          });
        });
        return gc;
      default:
        break;
    }
  });
</script>

<Meta title="Components/Calendar" component={CalendarList} />

<Template>
  <CalendarList />
</Template>

<Story name="Primary" args={{}} />
