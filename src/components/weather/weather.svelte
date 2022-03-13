<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onDestroy } from "svelte";
  import Card from "../../layout/card.svelte";
  import WeatherCard from "./weatherCard.svelte";
  type WeatherResponse = {
    main: {
      temp: number;
      humidity: number;
      feelsLike: number;
      tempMin: number;
      tempMax: number;
    };
  };

  let weather = invoke<WeatherResponse>("get_weather");
  let refreshedAt = new Date();

  let timer = setInterval(() => {
    weather = invoke<WeatherResponse>("get_weather");
    refreshedAt = new Date();
  }, 1000 * 60 * 5);

  onDestroy(async () => {
    clearInterval(timer);
  });
</script>

<Card>
  <h1 class="text-lg underline">Weather</h1>
  {#await weather}
    <p>loading...</p>
  {:then val}
    <WeatherCard
      at={refreshedAt}
      temp={+val.main.temp.toPrecision(2)}
      humidity={+val.main.humidity.toPrecision(2)}
      feelsLike={+val.main.feelsLike.toPrecision(2)}
      tempMax={+val.main.tempMax.toPrecision(2)}
      tempMin={+val.main.tempMin.toPrecision(2)}
    />
  {:catch err}
    <p>{err?.msg}</p>
  {/await}
</Card>
