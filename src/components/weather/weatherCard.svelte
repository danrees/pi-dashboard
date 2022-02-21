<script lang="ts">
  export let temp = 0;
  export let humidity = 0;
  export let at = new Date();
</script>

<div>
  <p>At: {at}</p>
  <p>Temperature: {temp}</p>
  <p>Humidity: {humidity}</p>
</div>
