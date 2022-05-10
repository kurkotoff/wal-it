<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { images_data } from "../stores";

    let userInput;

    async function searchWal() {
        var img_data_string = await invoke("search_wal", {query: userInput});
        var img_data_arr = JSON.parse(img_data_string)["data"];
        images_data.set(img_data_arr);
    }
</script>

<main>
    <input type="text" bind:value={userInput}/>
    <button on:click={searchWal}>Search</button>
</main>

<style>
    button {
        background-color: hsl(133, 70%, 36%);
        cursor: pointer;
        color: white;
        margin: 0.5rem;
        padding: 0.5em;
        width: 4.5em;
        height: 2.4em;
        font-size: 1em;
        border-radius: 0.2em;
        border:transparent
    }

    button:hover {
        background-color: hsl(133, 70%, 16%);
    }

    input {
        font-size: 1em;
        padding: 0.5em;
    }
</style>