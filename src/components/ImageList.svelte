<script>
    import Image from "./Image.svelte";
    import { images_data } from "../stores";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

</script>

<main>
    <ul>
        {#each $images_data as image}
            <li on:click={
                function showImageScreen() {
                    dispatch("set-image-screen", {viewImage: image});
                }
            }><Image src={image.thumbs.small} alt={image.id}/></li>
        {/each}
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
