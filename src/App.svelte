<script lang="ts">
  import Router from "svelte-spa-router";
  import routes from "./routes";

  let calendarList: {
    items: { summary: string }[];
  } = {
    items: [],
  };
</script>

<nav class="bg-blue-900 shadow-lg">
  <div class="container mx-auto">
    <div class="sm:flex">
      <a href="#/" class="text-white text-3xl font-bold p-3">APP LOGO</a>

      <!-- Menus -->
      <div class="ml-55 mt-4">
        <ul class="text-white sm:self-center text-xl">
          <li class="sm:inline-block">
            <a href="#/calendarlist" class="p-3 hover:text-red-500"
              >Select Calendar</a
            >
          </li>
        </ul>
      </div>
    </div>
  </div>
</nav>
<div>
  <Router {routes} />
</div>

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
