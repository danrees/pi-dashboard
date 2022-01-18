<script lang="ts">
  import { WebviewWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/tauri";

  import Agenda from "./components/agenda/agenda.svelte";
  import Weather from "./components/weather/weather.svelte";
  import { open } from "@tauri-apps/api/shell";

  const onLogin = async () => {
    try {
      await invoke("login");
    } catch (err) {
      console.log(err);
    }
  };
</script>

<nav class="bg-blue-900 shadow-lg">
  <div class="container mx-auto">
    <div class="sm:flex">
      <a href="#" class="text-white text-3xl font-bold p-3">APP LOGO</a>

      <!-- Menus -->
      <div class="ml-55 mt-4">
        <ul class="text-white sm:self-center text-xl">
          <li class="sm:inline-block">
            <a href="#" class="p-3 hover:text-red-900">About</a>
          </li>
          <li class="sm:inline-block">
            <a href="#" class="p-3 hover:text-red-900">Services</a>
          </li>
          <li class="sm:inline-block">
            <a href="#" class="p-3 hover:text-red-900">Blog</a>
          </li>
          <li class="sm:inline-block">
            <a href="#" class="p-3 hover:text-red-900">Contact</a>
          </li>
        </ul>
      </div>
    </div>
  </div>
</nav>
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

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
