<script lang="ts">
  export let temp = 0;
  export let humidity = 0;
  export let at = new Date();
  export let feelsLike = 0;
  export let tempMin = 0;
  export let tempMax = 0;
</script>

<div>
  <p>At: <span class="text-slate-500 text-sm">{at}</span></p>
  <p>Temperature: {temp}</p>
  <p>Humidity: {humidity}</p>
  <p>Feels Like: {feelsLike}</p>
  <p>Min: {tempMin}</p>
  <p>Max: {tempMax}</p>
</div>
