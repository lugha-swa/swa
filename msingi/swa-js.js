// swa-js.js -- kiolesura cha JS cha Swa. Kimetolewa na `swa --wasm-js`; usikihariri.
// Halina mantiki ya programu: Swa hufanya kila kitu kupitia uingizaji wa
// moduli 'js' (msingi/maktaba/kivinjari.swa). Kitu cha JS hushikwa kwa
// namba (kishiko): 0 = tupu, 1 = nodi kuu.
(function (nje) {
    'use strict';
    function unganisha(nodi_kuu) {
        const vitu = [undefined, nodi_kuu];
        const huru = [];
        const msimbuaji = new TextDecoder('utf-8');
        let kumbukumbu = null;
        let jedwali = null;
        function shika(x) {
            const h = huru.length > 0 ? huru.pop() : vitu.length;
            vitu[h] = x;
            return h;
        }
        function soma(p) {
            const b = new Uint8Array(kumbukumbu.buffer);
            let m = p;
            while (b[m] !== 0) { m = m + 1; }
            return msimbuaji.decode(b.subarray(p, m));
        }
        function achilia(h) {
            if (h > 1 && h < vitu.length && vitu[h] !== undefined) { vitu[h] = undefined; huru.push(h); }
        }
        const js = {
            nodi_kuu: () => 1,
            kimataifa: (jina) => shika(globalThis[soma(jina)]),
            pata: (h, sifa) => shika(vitu[h][soma(sifa)]),
            weka: (h, sifa, t) => { vitu[h][soma(sifa)] = vitu[t]; },
            ita0: (h, njia) => shika(vitu[h][soma(njia)]()),
            ita1: (h, njia, a) => shika(vitu[h][soma(njia)](vitu[a])),
            ita2: (h, njia, a, b) => shika(vitu[h][soma(njia)](vitu[a], vitu[b])),
            ita3: (h, njia, a, b, c) => shika(vitu[h][soma(njia)](vitu[a], vitu[b], vitu[c])),
            mfuatano: (p) => shika(soma(p)),
            namba: (n) => shika(n),
            kwa_namba: (h) => Number(vitu[h]) | 0,
            kazi: (k) => shika(function (...hoja) {
                const vs = hoja.map(shika);
                try { return jedwali.get(k)(...vs); } finally { vs.forEach(achilia); }
            }),
            achilia: achilia,
        };
        function weka_moduli(exports) {
            kumbukumbu = exports.memory;
            jedwali = exports.jedwali || null;
        }
        return { js, weka_moduli, vitu };
    }
    async function anzishaSwa(chanzo, nodi_kuu, ziada) {
        const baiti = typeof chanzo === 'string' ? await (await fetch(chanzo)).arrayBuffer() : chanzo;
        if (nodi_kuu === undefined && globalThis.document) { nodi_kuu = globalThis.document.body; }
        const d = unganisha(nodi_kuu);
        const uingizaji = Object.assign({ js: d.js }, ziada || {});
        const { instance } = await WebAssembly.instantiate(baiti, uingizaji);
        d.weka_moduli(instance.exports);
        const matokeo = instance.exports.anza();
        return { instance, matokeo, vitu: d.vitu };
    }
    nje.anzishaSwa = anzishaSwa;
    if (typeof module !== 'undefined' && module.exports) { module.exports = { anzishaSwa }; }
    const kianzio = globalThis.document ? globalThis.document.currentScript : null;
    if (kianzio && kianzio.dataset && kianzio.dataset.swa) {
        const anza_ukurasa = () => {
            const kitambulisho = kianzio.dataset.nodi;
            anzishaSwa(kianzio.dataset.swa, kitambulisho ? globalThis.document.getElementById(kitambulisho) : undefined);
        };
        if (globalThis.document.readyState === 'loading') { globalThis.document.addEventListener('DOMContentLoaded', anza_ukurasa); } else { anza_ukurasa(); }
    }
})(typeof globalThis !== 'undefined' ? globalThis : this);
