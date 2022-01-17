<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Card from "../../layout/card.svelte";
  type WeatherResponse = {
    main: {
      temp: number;
      humidity: number;
    };
  };
  let weather = invoke<WeatherResponse>("get_weather");
</script>

<Card>
  <h1>Weather</h1>
  {#await weather}
    <p>loading...</p>
  {:then val}
    <p>Temperature: {val.main.temp.toPrecision(4)}</p>
    <p>Humidity: {val.main.humidity}</p>
  {:catch err}
    <p>{err}</p>
  {/await}
</Card>
