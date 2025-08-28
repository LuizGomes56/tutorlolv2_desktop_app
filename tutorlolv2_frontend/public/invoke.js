const invoke = window.__TAURI_INTERNALS__?.invoke;

/**
 * 
 * @returns {boolean}
 */
export function invoke_checkup() {
    return typeof invoke === "function";
}

/**
 * 
 * @returns {Promise<string | Uint8Array>}
 */
export async function invoke_get_live_game() {
    if (!invoke) {
        return "Desktop application is not in use. Extern calls must not be performed";
    }
    /**
     * @type {Uint8Array | undefined}
     */
    let array = await invoke?.("get_live_game");

    if (!array) {
        return "No live game data";
    }

    let len = array.length;
    /**
     * @type {number}
     */
    let ptr = wasmBindings.alloc_live_game_buffer(len);
    console.log("Called mimalloc", ptr)
    new Uint8Array(window.wasm.memory.buffer, ptr, len).set(array);
    wasmBindings.parse_live_game(ptr, len);
}