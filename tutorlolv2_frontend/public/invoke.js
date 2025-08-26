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
    return await invoke?.("get_live_game");
}