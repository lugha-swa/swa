// dom_mock.js — DOM mock isiyo na tegemeo la nje (hakuna jsdom/
// puppeteer -- Node haina `document`), kwa majaribio ya Awamu 3
// (Sehemu ya Uingizaji + daraja la DOM). Hutoa importObject.env yenye
// kazi 6 zinazoshughulikia mti rahisi wa nodi (JS array ya kawaida)
// badala ya DOM halisi.
//
// Matumizi:
//   const dom = createDomMock();
//   const { instance } = await WebAssembly.instantiate(buf, dom.importObject);
//   dom.setMemory(instance.exports.memory);  // KABLA ya kuita main()
//   instance.exports.main();
//   dom.getTree();  // muundo wa mti tangu id=0 (mzizi)
//
// MUHIMU: importObject inajengwa KABLA ya instantiate (WASM inataka
// hivyo), LAKINI kazi zake zinahitaji kusoma kumbukumbu ya moduli
// (memory ni EXPORT, si import -- haipatikani mpaka baada ya
// instantiate kukamilika). Suluhisho la kawaida: `memory` ni kigezo
// cha nje (closure) kilichowekwa BAADAYE na setMemory() -- salama kwa
// sababu kazi za env.* hazitaitwa (WASM haiwezi) mpaka main() iitwe,
// jambo linalotokea BAADA ya setMemory().

function createDomMock() {
    let memory = null;
    const nodes = [{ tag: "#mzizi", attrs: {}, text: "", watoto: [] }];

    function somaMfuatano(ptr) {
        const bytes = new Uint8Array(memory.buffer);
        let end = ptr;
        while (bytes[end] !== 0) end++;
        return Buffer.from(bytes.slice(ptr, end)).toString("utf8");
    }

    const importObject = {
        env: {
            dom_unda: (tagPtr) => {
                const tag = somaMfuatano(tagPtr);
                const id = nodes.length;
                nodes.push({ tag, attrs: {}, text: "", watoto: [] });
                return id;
            },
            dom_weka_maandishi: (id, textPtr) => {
                nodes[id].text = somaMfuatano(textPtr);
            },
            dom_weka_sifa: (id, namePtr, valPtr) => {
                nodes[id].attrs[somaMfuatano(namePtr)] = somaMfuatano(valPtr);
            },
            dom_ambatanisha: (mzaziId, mtotoId) => {
                nodes[mzaziId].watoto.push(mtotoId);
            },
            dom_mzizi: () => 0,
            console_andika: (ptr) => {
                console.log(somaMfuatano(ptr));
            },
        },
    };

    function setMemory(mem) {
        memory = mem;
    }

    // Rudisha mti ulioanzia id (chaguo-msingi: mzizi, id=0) kama
    // muundo unaosomeka (tag/attrs/text/watoto) -- watoto ni MITI
    // YENYEWE iliyopanuliwa (si id tupu), kwa ulinganishi rahisi wa
    // JSON.stringify moja kwa moja dhidi ya muundo tarajiwa.
    function panuaMti(id) {
        const n = nodes[id];
        return {
            tag: n.tag,
            attrs: n.attrs,
            text: n.text,
            watoto: n.watoto.map(panuaMti),
        };
    }

    function getTree(id) {
        return panuaMti(id === undefined ? 0 : id);
    }

    return { importObject, setMemory, getTree };
}

module.exports = { createDomMock };
