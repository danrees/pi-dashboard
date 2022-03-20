<script lang="ts">
  import { Meta, Story, Template } from "@storybook/addon-svelte-csf";
  import { mockIPC } from "@tauri-apps/api/mocks";
  import Weather from "./weather.svelte";

  mockIPC((cmd, args) => {
    switch (cmd) {
      case "get_weather":
        const p = new Promise((resolve) => {
          resolve({
            main: {
              at: new Date(2022, 1, 1, 1, 1, 1),
              temp: 20,
              humidity: 60,
              feelsLike: 20,
              tempMin: 10,
              tempMax: 30,
            },
          });
        });
        return p;
      default:
        break;
    }
  });
</script>

<Meta title="Components/Weather" component={Weather} />

<Template let:args>
  <Weather {...args} />
</Template>

<Story name="Primary" args={{}} />
