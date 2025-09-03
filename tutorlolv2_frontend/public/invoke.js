const invoke = window.__TAURI_INTERNALS__?.invoke;

/**
 * 
 * @returns {boolean}
 */
export function invoke_checkup() {
    return typeof invoke === "function";
}

const ws = new WebSocket("ws://localhost:8082/api/games/realtime/ws");
ws.binaryType = "arraybuffer";

const ready = new Promise((resolve, reject) => {
    ws.addEventListener("open", () => resolve(ws), { once: true });
    ws.addEventListener("error", (event) => reject(event), { once: true });
});

let keepAliveTimer = null;
ready.then(() => {
    if (!keepAliveTimer) {
        keepAliveTimer = setInterval(() => {
            if (ws.readyState === WebSocket.OPEN) {
                try { ws.send("ping"); } catch { }
            }
        }, 25_000);
    }
});

let chain = Promise.resolve();

export function invoke_get_live_game() {
    chain = chain.then(async () => {
        if (!invoke) {
            console.warn("Desktop application is not in use. Extern calls must not be performed");
            return;
        }

        await ready;

        /** @type {Uint8Array | undefined} */
        const u8 = await invoke?.("get_live_game");
        if (!u8 || u8.byteLength === 0) {
            console.info("No live game data");
            return;
        }

        ws.send(u8);

        await new Promise((resolve, reject) => {
            let expectingHeader = true;
            let totalLen = 0;
            let ptr = 0;
            let written = 0;

            const onError = (ev) => { cleanup(); reject(ev instanceof Error ? ev : new Error("WebSocket error")); };
            const onClose = () => { cleanup(); reject(new Error("WebSocket closed during stream")); };

            const onMessage = (event) => {
                if (!(event.data instanceof ArrayBuffer)) return;

                /** @type {ArrayBuffer} */
                const serverData = event.data;

                if (expectingHeader) {
                    if (serverData.byteLength < 4) {
                        cleanup();
                        reject(new Error("Header frame shorter than 4 bytes"));
                        return;
                    }
                    try {
                        totalLen = new Uint32Array(serverData.slice(0, 4))[0];
                        console.log(`Receiving ${totalLen} bytes`);
                        ptr = wasmBindings.alloc_live_game_buffer(totalLen);
                    } catch (e) {
                        console.error(`Rejected promise: ${e}`);
                        cleanup();
                        reject(e);
                        return;
                    }

                    expectingHeader = false;

                    const remainder = serverData.byteLength - 4;
                    if (remainder > 0) {
                        console.log(`Receiving ${remainder} more bytes`);
                        const chunk = new Uint8Array(serverData, 4, remainder);
                        const view = new Uint8Array(window.wasm.memory.buffer, ptr + written, Math.min(chunk.byteLength, totalLen - written));
                        view.set(chunk.subarray(0, view.byteLength));
                        written += view.byteLength;
                    }
                } else {
                    const chunk = new Uint8Array(serverData);
                    const remaining = totalLen - written;
                    if (remaining <= 0) return;

                    const toCopy = Math.min(remaining, chunk.byteLength);
                    const view = new Uint8Array(window.wasm.memory.buffer, ptr + written, toCopy);
                    view.set(chunk.subarray(0, toCopy));
                    written += toCopy;
                }

                if (!expectingHeader && written === totalLen) {
                    try {
                        console.log(`Parsing ${written} bytes`);
                        wasmBindings.parse_live_game(ptr, written);
                    } catch (e) {
                        console.error(`[1] Rejected promise: ${e}`);
                        cleanup();
                        reject(e);
                        return;
                    }
                    cleanup();
                    resolve();
                }
            };

            const cleanup = () => {
                ws.removeEventListener("message", onMessage);
                ws.removeEventListener("error", onError);
                ws.removeEventListener("close", onClose);
            };

            ws.addEventListener("message", onMessage);
            ws.addEventListener("error", onError);
            ws.addEventListener("close", onClose);
        });
    });

    return chain;
}
