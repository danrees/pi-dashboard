<script lang="ts">
  import { Meta, Story, Template } from "@storybook/addon-svelte-csf";
  import { mockIPC } from "@tauri-apps/api/mocks";
  import Weather from "../../components/weather/weather.svelte";

  mockIPC((cmd, args) => {
    switch (cmd) {
      case "get_weather":
        const p = new Promise((resolve) => {
          resolve({
            main: { at: Date.now(), temp: 20, humidity: 60 },
          });
        });
        return p;
      default:
        break;
    }
  });
</script>

<Meta title="Weather" component={Weather} />

<Template let:args>
  <Weather {...args} />
</Template>

<Story name="Primary" args={{}} />
