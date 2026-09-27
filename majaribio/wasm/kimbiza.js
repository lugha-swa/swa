// kimbiza.js — kikaguzi cha Node kisicho na tegemezi la nje (hakuna
// npm package) kwa moduli za WASM zinazozalishwa na "stage1 --wasm".
// Matumizi: node kimbiza.js <faili.wasm> <thamani_tarajiwa> [aina]
//
// aina (hiari, chaguo-msingi "namba"):
//   namba     -- main() haihitaji hoja, matokeo (i32) yanalinganishwa
//                moja kwa moja na <thamani_tarajiwa> (namba kamili).
//                Tabia ya asili ya Awamu 1, HAIJABADILIKA.
//   mfuatano  -- main() inarudisha i32 (ofseti ya kumbukumbu, ptr).
//                Baiti zinasomwa kutoka instance.exports.memory.buffer
//                kuanzia ptr HADI baiti 0x00 (mkataba wa mfuatano wa
//                NUL-terminated, sawa na jinsi mfuatano halisi
//                unavyohifadhiwa na wasm_ongeza_mfuatano), kisha
//                kubadilishwa kuwa Latin1 na kulinganishwa na
//                <thamani_tarajiwa> (mfuatano halisi, SI namba).
//   safu      -- main() inarudisha i32 (ptr kwenye safu ya n32,
//                little-endian, kama WASM linear memory yenyewe).
//                <thamani_tarajiwa> ni namba kamili zilizotenganishwa
//                kwa mkato (,), idadi ya vipengele inatokana na hesabu
//                yao. Kila kipengele kinasomwa kama i32 (baiti 4) na
//                kulinganishwa moja kwa moja.
//
// Husoma faili, huthibitisha (WebAssembly.validate), huipakia
// (WebAssembly.instantiate), huita instance.exports.main(), na
// kulinganisha na thamani tarajiwa kulingana na aina. process.exit(0)
// kwa mafanikio, process.exit(1) kwa kushindwa kokote (haikubaliki,
// haijapakika, matokeo si sawa).

const fs = require("fs");

function shindwa(ujumbe) {
    console.error("SHINDWA: " + ujumbe);
    process.exit(1);
}

const njia = process.argv[2];
const tarajiwa_str = process.argv[3];
const aina = process.argv[4] || "namba";

if (!njia || tarajiwa_str === undefined) {
    shindwa("matumizi: node kimbiza.js <faili.wasm> <thamani_tarajiwa> [aina]");
}

let bafa;
try {
    bafa = fs.readFileSync(njia);
} catch (e) {
    shindwa("faili haifunguki: " + njia + " (" + e.message + ")");
}

if (!WebAssembly.validate(bafa)) {
    shindwa("moduli ya WASM haikubaliki (WebAssembly.validate): " + njia);
}

WebAssembly.instantiate(bafa)
    .then((matokeo) => {
        const main = matokeo.instance.exports.main;
        if (typeof main !== "function") {
            shindwa("hakuna 'main' iliyotolewa nje kwenye moduli: " + njia);
        }
        const halisi = main();

        if (aina === "namba") {
            const tarajiwa = parseInt(tarajiwa_str, 10);
            if (Number.isNaN(tarajiwa)) {
                shindwa("thamani tarajiwa si namba kamili: " + tarajiwa_str);
            }
            if (halisi !== tarajiwa) {
                shindwa(
                    "matokeo si sawa (" + njia + "): tarajiwa=" + tarajiwa + " halisi=" + halisi
                );
            }
            process.exit(0);
        }

        const mem = matokeo.instance.exports.memory;
        if (!mem) {
            shindwa("moduli haijatoa nje 'memory' (inahitajika kwa aina=" + aina + "): " + njia);
        }
        const bytes = new Uint8Array(mem.buffer);

        if (aina === "mfuatano") {
            let end = halisi;
            while (end < bytes.length && bytes[end] !== 0) end++;
            if (end >= bytes.length) {
                shindwa("mfuatano halikuisha na \\0 ndani ya kumbukumbu (" + njia + ")");
            }
            const halisi_str = Buffer.from(bytes.slice(halisi, end)).toString("latin1");
            if (halisi_str !== tarajiwa_str) {
                shindwa(
                    "mfuatano si sawa (" + njia + "): tarajiwa=" + JSON.stringify(tarajiwa_str) +
                        " halisi=" + JSON.stringify(halisi_str)
                );
            }
            process.exit(0);
        }

        if (aina === "safu") {
            const tarajiwa_arr = tarajiwa_str.split(",").map((s) => parseInt(s, 10));
            const view = new DataView(mem.buffer);
            for (let i = 0; i < tarajiwa_arr.length; i++) {
                const halisi_v = view.getInt32(halisi + i * 4, true);
                if (halisi_v !== tarajiwa_arr[i]) {
                    shindwa(
                        "safu si sawa (" + njia + ") kwenye kipengele " + i +
                            ": tarajiwa=" + tarajiwa_arr[i] + " halisi=" + halisi_v
                    );
                }
            }
            process.exit(0);
        }

        shindwa("aina ya jaribio haijulikani: " + aina);
    })
    .catch((e) => {
        shindwa("moduli haikupakika/kuendeshwa (" + njia + "): " + e.message);
    });
