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
//   dom       -- (Awamu 3) main() haihitaji thamani ya kurudi maalum
//                (inaweza kuwa 0/w0) -- baada ya kuiendesha, mti wa
//                dom_mock.js (kuanzia mzizi, id=0) unalinganishwa
//                (JSON.stringify iliyopangwa) na muundo tarajiwa
//                uliohifadhiwa kwenye FAILI la JSON ambalo NJIA yake
//                ni <thamani_tarajiwa> (safu ya kwanza ya MANIFEST,
//                si namba -- angalia majaribio/wasm/*.tarajiwa.json).
//
// Sehemu za Uingizaji (Awamu 3): KILA moduli sasa ina Import section
// (kazi 6 za env.*, angalia dom_mock.js) -- importObject hutolewa
// kwa KILA aina ya jaribio, si "dom" pekee, kwa sababu
// WebAssembly.instantiate inakataa moduli yenye uingizaji bila
// importObject inayolingana, hata kama uingizaji huo haitumiki
// kamwe wakati wa kukimbia.
//
// Husoma faili, huthibitisha (WebAssembly.validate), huipakia
// (WebAssembly.instantiate na importObject ya dom_mock), huita
// instance.exports.main(), na kulinganisha na thamani tarajiwa
// kulingana na aina. process.exit(0) kwa mafanikio, process.exit(1)
// kwa kushindwa kokote (haikubaliki, haijapakika, matokeo si sawa).

const fs = require("fs");
const path = require("path");
const { createDomMock } = require("./dom_mock.js");

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

const dom = createDomMock();

WebAssembly.instantiate(bafa, dom.importObject)
    .then((matokeo) => {
        const main = matokeo.instance.exports.main;
        if (typeof main !== "function") {
            shindwa("hakuna 'main' iliyotolewa nje kwenye moduli: " + njia);
        }
        if (matokeo.instance.exports.memory) {
            dom.setMemory(matokeo.instance.exports.memory);
        }
        const halisi = main();

        if (aina === "dom") {
            let tarajiwa_tree;
            try {
                const jp = path.isAbsolute(tarajiwa_str) ? tarajiwa_str : path.join(__dirname, tarajiwa_str);
                tarajiwa_tree = JSON.parse(fs.readFileSync(jp, "utf8"));
            } catch (e) {
                shindwa("faili la mti tarajiwa halisomeki (" + tarajiwa_str + "): " + e.message);
            }
            const halisi_tree = dom.getTree();
            const a = JSON.stringify(halisi_tree);
            const b = JSON.stringify(tarajiwa_tree);
            if (a !== b) {
                shindwa(
                    "mti wa DOM si sawa (" + njia + "):\n  tarajiwa=" + b + "\n  halisi=  " + a
                );
            }
            process.exit(0);
        }

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
