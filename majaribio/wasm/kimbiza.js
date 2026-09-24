// kimbiza.js — kikaguzi cha Node kisicho na tegemezi la nje (hakuna
// npm package) kwa moduli za WASM zinazozalishwa na "stage1 --wasm".
// Matumizi: node kimbiza.js <faili.wasm> <thamani_tarajiwa>
//
// Husoma faili, huthibitisha (WebAssembly.validate), huipakia
// (WebAssembly.instantiate), huita instance.exports.main() bila
// hoja, na kulinganisha matokeo na thamani tarajiwa. process.exit(0)
// kwa mafanikio, process.exit(1) kwa kushindwa kokote (haikubaliki,
// haijapakika, matokeo si sawa).

const fs = require("fs");

function shindwa(ujumbe) {
    console.error("SHINDWA: " + ujumbe);
    process.exit(1);
}

const njia = process.argv[2];
const tarajiwa_str = process.argv[3];

if (!njia || tarajiwa_str === undefined) {
    shindwa("matumizi: node kimbiza.js <faili.wasm> <thamani_tarajiwa>");
}

const tarajiwa = parseInt(tarajiwa_str, 10);
if (Number.isNaN(tarajiwa)) {
    shindwa("thamani tarajiwa si namba kamili: " + tarajiwa_str);
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
        if (halisi !== tarajiwa) {
            shindwa(
                "matokeo si sawa (" +
                    njia +
                    "): tarajiwa=" +
                    tarajiwa +
                    " halisi=" +
                    halisi
            );
        }
        process.exit(0);
    })
    .catch((e) => {
        shindwa("moduli haikupakika/kuendeshwa (" + njia + "): " + e.message);
    });
