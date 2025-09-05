/**
 * 
 * @param {number} ptr 
 * @returns {void}
 */
async function call_calculator(ptr) {
    const wasm = window.wasm;
    const memView = new Uint8Array(wasm.memory.buffer, ptr, len);

    const response = await fetch("http://localhost:8082/api/games/calculator", {
        method: "POST",
        headers: {
            "Content-Type": "application/octet-stream",
            "Accept": "application/octet-stream"
        },
        body: memView
    });

    const len = Number(response.headers.get("content-length"));
    const buffer = await response.arrayBuffer();
    len = buffer.byteLength;

    /**
     * @type {number}
     */
    const allocPtr = wasmBindings.alloc_bytes(len);

    /**
     * @type {WebAssembly.Memory}
     */
    const mem = wasm.memory;
    new Uint8Array(mem.buffer, allocPtr, len).set(new Uint8Array(buffer));
    wasmBindings.parse_calculator(allocPtr, len);
}