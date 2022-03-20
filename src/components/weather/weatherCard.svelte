<script lang="ts">
  import Display from "./display.svelte";

  export let temp = 0;
  export let humidity = 0;
  export let at = new Date();
  export let feelsLike = 0;
  export let tempMin = 0;
  export let tempMax = 0;
</script>

<div>
  <p>At: <Display>{at}</Display></p>
  <p>Temperature: <Display>{temp}&#176;C</Display></p>
  <p>Humidity: <Display>{humidity}</Display></p>
  <p>Feels Like: <Display>{feelsLike}&#176;C</Display></p>
  <p>Min: <Display>{tempMin}&#176;C</Display></p>
  <p>Max: <Display>{tempMax}&#176;C</Display></p>
</div>
