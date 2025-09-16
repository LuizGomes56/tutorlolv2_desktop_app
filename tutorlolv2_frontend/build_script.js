import { readFileSync, writeFileSync } from "fs";
import { resolve } from "path";
import { fileURLToPath } from "url";
import { dirname } from "path";

const __dirname = dirname(fileURLToPath(import.meta.url));

function loadEnv(path = ".env") {
    try {
        const lines = readFileSync(resolve(__dirname, path), "utf-8").split("\n");
        for (const line of lines) {
            const trimmed = line.trim();
            if (!trimmed || trimmed.startsWith("#")) continue;

            const [key, ...vals] = trimmed.split("=");
            if (key && !(key in process.env)) {
                process.env[key] = vals.join("=").trim().replace(/^['"]|['"]$/g, "");
            }
        }
    } catch (err) {
        console.warn(`.env not found: ${err.message}`);
    }
}

loadEnv();

const res = await fetch("http://localhost:8082/api/static/comptime", {
    method: "GET",
    headers: {
        "Content-Type": "application/json",
        "Accept": "text/plain",
    },
});

if (!res.ok) {
    console.error("Error when searching:", res.status, await res.text());
    process.exit(1);
}

const code = await res.text();
writeFileSync("tutorlolv2_imports/src/code.rs", code);
