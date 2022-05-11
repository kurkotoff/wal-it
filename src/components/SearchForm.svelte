<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { query_input, img_data, img_meta } from "../stores";
    import Image from "./Image.svelte";
    import { createEventDispatcher } from "svelte";
    import IntersectionObserver from "svelte-intersection-observer";

    const dispatch = createEventDispatcher();

    let element;
    let page = 1;

    function testScroll() {
        console.log("Scrolled to bottom");
    }

    async function getImgData() {
        var img_data_string = await invoke("search_wal", {query: $query_input, page: page});
        var img_data_arr = JSON.parse(img_data_string)["data"];
        $img_meta = JSON.parse(img_data_string)["meta"];

        return img_data_arr
    }

    async function setImgData() {
        page = 1;
        var new_data = await getImgData();
        $img_data = new_data;
    }

    async function updateImgData() {
        if ($query_input == $img_meta.query && $query_input != "") {
            page++;
            var new_data = await getImgData();
            $img_data = $img_data.concat(new_data);
        }
    }
</script>

<main>
    <input type="text" bind:value={$query_input}/>
    <button on:click={setImgData}>Search</button>

    <ul>
        {#each $img_data as image}
            <li on:click={
                function showImageScreen() {
                    dispatch("set-image-screen", {viewImage: image});
                }
            }><Image src={image.thumbs.small} alt={image.id}/></li>
        {/each}
        
        <!-- TODO: This shows "undefined" for some reason. Should fix -->
        <IntersectionObserver {element} on:intersect={updateImgData}>
            <div bind:this={element} style="visibility: hidden;"/>
        </IntersectionObserver>
    </ul>
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

    ul {
        list-style-type: none;
    }

    li {
        display: inline-block;
        margin: 0.5em;
        cursor: pointer;
    }
</style>