// swa-kivinjari.js -- daraja la kivinjari kwa moduli za WASM za Swa
// (`swa --wasm p.swa > p.wasm`). Huunganisha uingizaji 8 wa env.* na
// DOM halisi, kisha huita main().
//
//   <div id="swa"></div>
//   <script src="swa-kivinjari.js"></script>
//   <script>anzishaSwa("p.wasm", document.getElementById("swa"));</script>
//
// Nodi zinatambuliwa kwa namba (id): 0 ni `mzizi` uliopewa, kila
// dom_unda huongeza moja. Msikilizaji (dom_sikiliza) huita kazi ya Swa
// kupitia jedwali la kazi lililotolewa nje na moduli: jedwali.get(kazi)(id).
// Kivinjari hakiruhusu fetch ya file:// -- tumikia saraka kwa seva
// yoyote ya HTTP (mf. `python3 -m http.server`).
(function (nje) {
    "use strict";

    function unganisha(mzizi, hati) {
        const nodi = [mzizi];
        const msimbuaji = new TextDecoder("utf-8");
        let kumbukumbu = null;
        let jedwali = null;

        function mfuatano(ptr) {
            const baiti = new Uint8Array(kumbukumbu.buffer);
            let mwisho = ptr;
            while (baiti[mwisho] !== 0) mwisho++;
            return msimbuaji.decode(baiti.subarray(ptr, mwisho));
        }

        const env = {
            dom_unda: (tagPtr) => nodi.push(hati.createElement(mfuatano(tagPtr))) - 1,
            dom_weka_maandishi: (id, ptr) => { nodi[id].textContent = mfuatano(ptr); },
            dom_weka_sifa: (id, jina, thamani) => { nodi[id].setAttribute(mfuatano(jina), mfuatano(thamani)); },
            dom_ambatanisha: (mzazi, mtoto) => { nodi[mzazi].appendChild(nodi[mtoto]); },
            dom_mzizi: () => 0,
            console_andika: (ptr) => { console.log(mfuatano(ptr)); },
            dom_sikiliza: (id, tukioPtr, kazi) => {
                nodi[id].addEventListener(mfuatano(tukioPtr), () => { jedwali.get(kazi)(id); });
            },
            dom_weka_namba: (id, n) => { nodi[id].textContent = String(n); },
        };

        function weka(exports) {
            kumbukumbu = exports.memory;
            jedwali = exports.jedwali || null;
        }

        return { importObject: { env }, weka, nodi };
    }

    // chanzo: URL ya .wasm, au baiti zake (ArrayBuffer / Uint8Array).
    async function anzishaSwa(chanzo, mzizi, hati) {
        hati = hati || nje.document;
        mzizi = mzizi || hati.body;
        const baiti = typeof chanzo === "string"
            ? await (await fetch(chanzo)).arrayBuffer()
            : chanzo;
        const daraja = unganisha(mzizi, hati);
        const { instance } = await WebAssembly.instantiate(baiti, daraja.importObject);
        daraja.weka(instance.exports);
        const matokeo = instance.exports.main();
        return { instance, matokeo, nodi: daraja.nodi };
    }

    nje.anzishaSwa = anzishaSwa;
    if (typeof module !== "undefined" && module.exports) {
        module.exports = { anzishaSwa };
    }
})(typeof globalThis !== "undefined" ? globalThis : this);
