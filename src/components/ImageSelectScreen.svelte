<script>
    export let image;
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    function dispatchCloseScreen() {
        dispatch("close-image-screen");
    }

    function dispatchDownloadImage() {
        dispatch("download-image", {path: image.path})
    }
</script>

<main>
    <div class="show-image">
        <img src={image.path} alt={image.id}>
    </div>
    <button class="back-btn" on:click={dispatchCloseScreen}>Back</button>
    <button class="wal-btn" on:click={dispatchDownloadImage}>Wal it!</button>
</main>


<style>
    main {
        background-color: rgba(24, 24, 24);
        text-align: center;
    }

    .show-image {
        border-left-width: 10em;
        position: relative;
        min-height: 100%;
        min-width: 800px;
    }

    button {
        cursor: pointer;
        margin: 0.5rem;
        padding: 0.5em;
        width: 4.5em;
        height: 2.4em;
        font-size: 1em;
        border-radius: 0.2em;
        border:transparent
    }

    .back-btn {
        background-color: hsl(210, 5%, 15%);
        color: white;
    }

    .back-btn:hover {
        background-color: hsl(210, 5%, 12%);
        color: hsl(0, 0%, 50%);
    }

    .wal-btn {
        background-color: hsl(133, 70%, 36%);
        color: white
    }

    .wal-btn:hover {
        background-color: hsl(133, 70%, 16%);
    }

    img {
        max-width: 80%;
        max-height: 80%;
    }
</style>