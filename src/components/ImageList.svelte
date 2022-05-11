<script>
    import Image from "./Image.svelte";
    import { createEventDispatcher } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { query_input, img_data, img_meta } from "../stores";
    import { onMount } from "svelte";
    import InfiniteLoading from "svelte-infinite-loading";

    const dispatch = createEventDispatcher();

    function testScroll() {
        console.log("Scrolled to bottom");
    }

    async function getImgData() {
        var img_data_string = await invoke("search_wal", {query: $query_input});
        var img_data_arr = JSON.parse(img_data_string)["data"];
        $img_meta = JSON.parse(img_data_string)["meta"];

        return img_data_arr
    }

    onMount(async () => {
        var new_data = await getImgData();
        if ($query_input == $img_meta.query) {
            $img_data = $img_data.concat(new_data);
        } else {
            $img_data = new_data;
        }
    })
</script>

<main>
    <ul>
        {#each $img_data as image}
            <li on:click={
                function showImageScreen() {
                    dispatch("set-image-screen", {viewImage: image});
                }
            }><Image src={image.thumbs.small} alt={image.id}/></li>
        {/each}
        <InfiniteLoading on:infinite={testScroll}/>
    </ul>
</main>

<style>
    ul {
        list-style-type: none;
    }

    li {
        display: inline-block;
        margin: 0.5em;
        cursor: pointer;
    }
</style>
