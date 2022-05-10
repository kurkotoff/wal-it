<script>
  import ImageList from "./components/ImageList.svelte";
  import SearchForm from "./components/SearchForm.svelte";
  import Navbar from "./components/Navbar.svelte";
  import ImageSelectScreen from "./components/ImageSelectScreen.svelte";
import { invoke } from "@tauri-apps/api/tauri";

  let view_image;

  function setImageScreen(event) {
    view_image = event.detail.viewImage;
  }

  function closeImageScreen(event) {
    view_image = undefined;
  }

  function downloadImage(event) {
    invoke("wal_image", {path: event.detail.path})
  }
</script>

<main>
  {#if view_image == undefined}
    <Navbar/>
    <SearchForm/>
    <ImageList on:set-image-screen={setImageScreen}/>
  {:else}
    <ImageSelectScreen image={view_image} on:close-image-screen={closeImageScreen} on:download-image={downloadImage}/>
  {/if}
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;

    background-color: rgb(24, 24, 24);
  }

  main {
    text-align: center;
    color: white;
    padding: 1em;
    padding-top: 5rem;
    margin: -1em -1em;
  }
</style>
