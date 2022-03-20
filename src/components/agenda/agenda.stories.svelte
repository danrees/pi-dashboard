<script lang="ts">
  import { Meta, Template, Story } from "@storybook/addon-svelte-csf";
  import Agenda from "../../components/agenda/agenda.svelte";
  import { mockIPC } from "@tauri-apps/api/mocks";

  mockIPC((cmd, args) => {
    switch (cmd) {
      case "get_events":
        return {
          items: [
            {
              id: "1",
              summary: "This is an event",
              start: {
                dateTime: new Date(2022, 1, 1, 1, 1, 1, 1),
              },
            },
            {
              id: "2",
              summary: "This is another event",
              start: {
                dateTime: Date.now(),
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

<Meta title="Components/Agenda" component={Agenda} />

<Template let:args>
  <Agenda {...args} />
</Template>

<Story name="Primary" args={{}} />
