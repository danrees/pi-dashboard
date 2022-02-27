<script lang="ts">
  import { Meta, Template, Story } from "@storybook/addon-svelte-csf";
  import Agenda from "../../components/agenda/agenda.svelte";
  import { mockIPC } from "@tauri-apps/api/mocks";
  import "../../app.css";

  mockIPC((cmd, args) => {
    switch (cmd) {
      case "get_events":
        return {
          items: [
            {
              id: "1",
              summary: "This is an event",
              start: {
                dateTime: new Date(),
              },
            },
          ],
        };
        break;
      default:
        break;
    }
  });
</script>

<Meta title="Agenda" component={Agenda} />

<Template let:args>
  <Agenda {...args} />
</Template>

<Story name="Primary" args={{}} />

<style lang="postcss">
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
