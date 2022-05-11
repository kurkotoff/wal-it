import { writable } from "svelte/store";

export const query_input = writable("");
export const img_data = writable([]);
export const img_meta = writable({query: ""});
export const current_view_image = writable();