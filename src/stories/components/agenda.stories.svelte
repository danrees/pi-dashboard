<script lang="ts">
  import { Meta, Template, Story } from "@storybook/addon-svelte-csf";
  import Agenda from "../../components/agenda/agenda.svelte";
  import { mockIPC } from "@tauri-apps/api/mocks";
  import Tailwindcss from "../Tailwindcss.svelte";

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
            {
              id: "2",
              summary: "This is another event",
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
  <Tailwindcss>
    <Agenda {...args} />
  </Tailwindcss>
</Template>

<Story name="Primary" args={{}} />

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
