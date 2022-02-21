<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Card from "../../layout/card.svelte";
  import WeatherCard from "./weatherCard.svelte";
  type WeatherResponse = {
    main: {
      temp: number;
      humidity: number;
    };
  };

  let weather = invoke<WeatherResponse>("get_weather");
  let refreshedAt = new Date();

  setInterval(() => {
    weather = invoke<WeatherResponse>("get_weather");
    refreshedAt = new Date();
  }, 1000 * 60 * 5);
</script>

<Card>
  <h1>Weather</h1>
  {#await weather}
    <p>loading...</p>
  {:then val}
    <WeatherCard
      at={refreshedAt}
      temp={+val.main.temp.toPrecision(2)}
      humidity={val.main.humidity}
    />
  {:catch err}
    <p>{err}</p>
  {/await}
</Card>
