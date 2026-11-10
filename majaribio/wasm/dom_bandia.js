// dom_bandia.js -- hati (document) bandia isiyo na tegemeo la nje (hakuna
// jsdom/puppeteer -- Node haina `document`), kwa majaribio ya WASM.
//
// Kiolesura cha mashine (awamu 4): programu ya Swa haiiti DOM moja kwa moja.
// msingi/maktaba/kivinjari.swa huita uingizaji wa moduli 'js' wa
// kiolesura kinachozalishwa na `swa --wasm-js`, nalo huita API hizi za DOM:
// createElement, appendChild, setAttribute, textContent,
// addEventListener. Mock hii hutoa API hizo pekee -- hivyo majaribio
// hupima mnyororo mzima: Swa -> kivinjari.swa -> swa-js.js -> DOM.
//
// Nodi hupewa namba kwa mpangilio wa kuundwa (nodi kuu = 0), kama mti
// tarajiwa wa majaribio/wasm/*.tarajiwa.json unavyozitaja.
//
//   const dom = createDomMock();
//   globalThis.document = dom.hati;
//   await anzishaSwa(baiti, dom.nodi_kuu);
//   dom.chochea(1, "click");
//   dom.getTree();

class Elementi {
    constructor(tag, id) {
        this.tag = tag;
        this.id_ya_jaribio = id;
        this.attrs = {};
        this.textContent = "";
        this.watoto = [];
        this.wasikilizaji = {};
    }
    setAttribute(k, v) { this.attrs[k] = String(v); }
    appendChild(m) { this.watoto.push(m); return m; }
    addEventListener(t, f) { (this.wasikilizaji[t] = this.wasikilizaji[t] || []).push(f); }
}

function createDomMock() {
    const nodi = [new Elementi("#nodi_kuu", 0)];
    const hati = {
        createElement: (tag) => {
            const e = new Elementi(tag, nodi.length);
            nodi.push(e);
            return e;
        },
    };

    // Chochea tukio kwenye nodi id -- kila msikilizaji huitwa kama
    // kivinjari kingefanya. Hurudisha idadi ya wasikilizaji walioitwa.
    function chochea(id, tukio) {
        const wasikilizaji = (nodi[id] && nodi[id].wasikilizaji[tukio]) || [];
        for (const f of wasikilizaji) f({ type: tukio, target: nodi[id] });
        return wasikilizaji.length;
    }

    function panuaMti(e) {
        return {
            tag: e.tag,
            attrs: e.attrs,
            text: String(e.textContent),
            watoto: e.watoto.map(panuaMti),
        };
    }

    function getTree() {
        return panuaMti(nodi[0]);
    }

    return { hati, nodi_kuu: nodi[0], chochea, getTree };
}

module.exports = { createDomMock };
