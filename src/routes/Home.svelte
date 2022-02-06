<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import Agenda from "../components/agenda/agenda.svelte";
  import Weather from "../components/weather/weather.svelte";

  const onLogin = async () => {
    try {
      await invoke("login");
    } catch (err) {
      console.log(err);
    }
  };
</script>

<main class="container mx-auto">
  <div>
    <button on:click={onLogin} class="rounded-md p-1 text-slate-300 bg-blue-500"
      >Login</button
    >
  </div>
  <div class="flex columns-2 gap-4">
    <Agenda />
    <Weather />
  </div>
</main>
