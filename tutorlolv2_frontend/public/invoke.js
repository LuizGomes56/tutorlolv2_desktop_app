const invoke = window.__TAURI_INTERNALS__?.invoke;

/**
 * 
 * @returns {boolean}
 */
export function invoke_checkup() {
    return typeof invoke === "function";
}

export async function invoke_get_live_game() {
    if (!invoke) {
        console.warn("Desktop application is not in use. Extern calls must not be performed");
        return;
    }

    /** @type {Uint8Array | undefined} */
    const body = await invoke?.("get_live_game");
    if (!body || body.byteLength === 0) {
        console.info("No live game data");
        return;
    }

    const response = await fetch("http://localhost:8082/api/games/realtime", {
        method: "POST",
        keepalive: true,
        cache: "no-cache",
        headers: {
            "Content-Type": "application/octet-stream",
            "Accept": "application/octet-stream",
        },
        body
    })

    const arrayBuffer = await response.arrayBuffer();
    const len = arrayBuffer.byteLength;
    const ptr = wasmBindings.alloc_bytes(len);

    const wasmView = new Uint8Array(window.wasm.memory.buffer, ptr, len);
    wasmView.set(new Uint8Array(arrayBuffer));

    wasmBindings.parse_live_game(ptr, len);
}
