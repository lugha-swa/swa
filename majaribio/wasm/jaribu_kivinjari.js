// jaribu_kivinjari.js -- jaribio la msingi/kivinjari/swa-kivinjari.js
// (daraja la KIVINJARI halisi, si dom_mock) bila kivinjari: hati
// bandia yenye createElement/appendChild/setAttribute/textContent/
// addEventListener tu -- yaani API ambazo daraja linatumia.
// Matumizi: node jaribu_kivinjari.js <kaunta.wasm>
"use strict";
const fs = require("fs");
const path = require("path");
const { anzishaSwa } = require(path.join(__dirname, "../../msingi/kivinjari/swa-kivinjari.js"));

function shindwa(u) { console.error("SHINDWA: " + u); process.exit(1); }

class Elementi {
    constructor(tag) { this.tag = tag; this.attrs = {}; this.textContent = ""; this.watoto = []; this.wasikilizaji = {}; }
    setAttribute(k, v) { this.attrs[k] = v; }
    appendChild(m) { this.watoto.push(m); return m; }
    addEventListener(t, f) { (this.wasikilizaji[t] = this.wasikilizaji[t] || []).push(f); }
    bofya() { for (const f of this.wasikilizaji.click || []) f(); }
}
const hati = { createElement: (t) => new Elementi(t) };
const mzizi = new Elementi("div");

anzishaSwa(fs.readFileSync(process.argv[2]), mzizi, hati).then(({ matokeo }) => {
    if (matokeo !== 0) shindwa("main() ilirudisha " + matokeo);
    const [kitufe, lebo, futa] = mzizi.watoto;
    if (!kitufe || kitufe.tag !== "button" || kitufe.textContent !== "Ongeza") shindwa("kitufe hakikujengwa sawa");
    if (lebo.textContent !== "0") shindwa("lebo ya mwanzo: " + JSON.stringify(lebo.textContent));
    kitufe.bofya(); kitufe.bofya(); kitufe.bofya();
    if (lebo.textContent !== "3") shindwa("baada ya mibofyo 3: " + lebo.textContent);
    futa.bofya(); kitufe.bofya();
    if (lebo.textContent !== "1") shindwa("baada ya kusafisha na mbofyo 1: " + lebo.textContent);
    console.log("kivinjari: kaunta 0 -> 3 -> 0 -> 1, sawa");
}).catch((e) => shindwa(e.stack || String(e)));
