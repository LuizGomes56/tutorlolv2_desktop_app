const invoke = window.__TAURI_INTERNALS__?.invoke;

/**
 * 
 * @returns {Uint8Array}
 */
export async function invoke_get_live_game() {
    if (!invoke) {
        console.warn("Desktop application is not in use. Extern calls must not be performed");
        return;
    }

    console.log("called invoke_get_live_game");

    /** @type {ArrayBuffer} */
    let data = await invoke?.("get_live_game");

    console.log("got data", data);

    return new Uint8Array(data);
}
