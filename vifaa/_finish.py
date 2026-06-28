"""Complete parser splitting for msambazaji.swa."""
with open('msingi/msambazaji.swa') as f: c = f.read()
added = ''

def extract_section(text, start_marker, end_marker, helper_name, call_label):
    """Extract section between markers, replace with helper call."""
    global c, added
    s = text.find(start_marker)
    e = text.find(end_marker)
    if s < 0 or e < 0:
        print(f'  SKIP {helper_name}: markers not found (s={s} e={e})')
        return False
    body = text[s:e].rstrip()
    added += f'N32 {helper_name}(Msambazaji* p) {{\n{body}\n    rudisha -1;\n}}\n\n'
    short = helper_name.split('_')[-1]
    text = text[:s] + f'    // {call_label}\n    N32 r_{short} = {helper_name}(p);\n    kama (r_{short} != -1) {{ rudisha r_{short}; }}\n' + text[e:]
    print(f'  {helper_name}: extracted {len(body.splitlines())} lines')
    return text

# === STEP 1: taarifa handlers ===
print('=== taarifa splits ===')
c = extract_section(c,
    '    // ---- kama ( cond ) block [ sivyo block ] ----',
    '    // ---- wakati ( cond ) block ----',
    'changanua_taarifa_kama', 'kama via helper')
c = extract_section(c,
    '    // ---- wakati ( cond ) block ----',
    '    // ---- chagua ( expr )',
    'changanua_taarifa_wakati', 'wakati via helper')
c = extract_section(c,
    '    // ---- chagua ( expr ) { hali X : ... [hali Y : ...] } ----',
    '    // ---- kwa ( init ; cond ; step ) block',
    'changanua_taarifa_chagua', 'chagua via helper')
c = extract_section(c,
    '    // ---- kwa ( init ; cond ; step ) block   (for loop) ----',
    '    // ---- Type-prefixed local declaration:',
    'changanua_taarifa_kwa', 'kwa via helper')
c = extract_section(c,
    '    // ---- Type-prefixed local declaration: N32 x ;  /  N32 x = expr ; ----',
    '    // ---- Expression statement  expr ; ----',
    'changanua_taarifa_tangazo', 'tangazo via helper')

# ndani: small handlers before the helper calls
s_ndani = c.find('    // ---- vunja ; ----')
e_ndani = c.find('    // kama via helper')
if s_ndani >= 0 and e_ndani >= 0:
    body = c[s_ndani:e_ndani].rstrip()
    added += f'N32 changanua_taarifa_ndani(Msambazaji* p) {{\n{body}\n    rudisha -1;\n}}\n\n'
    c = c[:s_ndani] + f'    // ndani via helper\n    N32 r_ndani = changanua_taarifa_ndani(p);\n    kama (r_ndani != -1) {{ rudisha r_ndani; }}\n\n' + c[e_ndani:]
    print('  changanua_taarifa_ndani: small handlers extracted')

# === STEP 2: kazi splits ===
print('=== kazi splits ===')
s_kazi = c.find('    // Parse parameter list')
e_kazi = c.find('    N32 func = node_mpya(AST_KAZI, ret_aina, name_node, params);')
if s_kazi >= 0 and e_kazi >= 0:
    kbody = c[s_kazi:e_kazi]
    v_end = kbody.find('    // Parse body statements')
    vpart = kbody[:v_end] + '    kama (p->sasa.aina == TOKENI_ISHARA && neno_ni(&p->sasa, "{")) { sogeza(p); }\n    rudisha params;'
    mpart = kbody[v_end:] + '\n    rudisha body;'
    added += f'N32 changanua_kazi_vigezo(Msambazaji* p) {{\n{vpart}\n}}\n\n'
    added += f'N32 changanua_kazi_mwili(Msambazaji* p) {{\n{mpart}\n}}\n\n'
    c = c[:s_kazi] + '    N32 params = changanua_kazi_vigezo(p);\n    kama (p->kosa != 0) { rudisha -1; }\n    N32 body = changanua_kazi_mwili(p);\n\n    N32 func = node_mpya(AST_KAZI, ret_aina, name_node, params);' + c[e_kazi:]
    print('  kazi vigezo+mwili extracted')

# === STEP 3: primary split ===
print('=== primary splits ===')
s_prim = c.find('    // Initialiser list  { elem , ... }')
e_prim = c.find('    // Prefix logical NOT')
if s_prim >= 0 and e_prim >= 0:
    body = c[s_prim:e_prim].rstrip()
    added += f'N32 changanua_primary_orodha(Msambazaji* p) {{\n{body}\n    rudisha -1;\n}}\n\n'
    c = c[:s_prim] + '    N32 r_orodha = changanua_primary_orodha(p);\n    kama (r_orodha != -1) { rudisha r_orodha; }\n\n    // Prefix logical NOT' + c[e_prim:]
    print('  primary orodha extracted')

# === STEP 4: split ndani further (extract achilia+tenga+rudisha) ===
print('=== further ndani split ===')
s2 = c.find('    // ---- achilia expr ; ----')
e2 = c.find('    rudisha -1;\n}\n\nN32 changanua_taarifa(Msambazaji* p) {')
if s2 >= 0 and e2 >= 0:
    body = c[s2:e2].rstrip()
    # Add to helpers, but need to check it doesn't include 'rudisha -1' from function end
    added += f'N32 changanua_ndani_kubwa(Msambazaji* p) {{\n{body}\n    rudisha -1;\n}}\n\n'
    c = c[:s2] + f'    N32 r_kubwa = changanua_ndani_kubwa(p);\n    kama (r_kubwa != -1) {{ rudisha r_kubwa; }}\n    rudisha -1;\n}}\n\n' + c[e2+len('    rudisha -1;\n}\n\n'):]
    print('  ndani further split done')

# === STEP 5: split kwa further (extract desugaring) ===
print('=== further kwa split ===')
old_desugar = '''            // Desugar for-loop into an init statement followed by a while loop.
            // The while loop wraps cond + (body + step appended to end of body).
            kama (step != -1) {
                kama (body != -1) {
                    // Find last statement in body chain.
                    N32 last = body;
                    wakati (ast_nne[last] != -1) { last = ast_nne[last]; }
                    ast_nne[last] = step;
                } sivyo {
                    body = step;
                }
            }

            N32 wakati_node = node_mpya(AST_WAKATI, 0, cond, body);

            kama (init != -1) {
                // Find tail of init if it chains (unlikely for single expr, but safe).
                ast_nne[init] = wakati_node;
                rudisha init;
            } sivyo {
                rudisha wakati_node;
            }'''
if old_desugar in c:
    added += f'N32 changanua_kwa_malizia(Msambazaji* p, N32 init, N32 cond, N32 step, N32 body) {{\n{old_desugar}\n}}\n\n'
    c = c.replace(old_desugar, '            N32 r = changanua_kwa_malizia(p, init, cond, step, body); rudisha r;')
    print('  kwa desugar extracted')

# === Insert all helpers and forward declarations ===
# Insert helpers before changanua_primary (first real function after forward decls)
ins_pos = c.find('\nN32 changanua_primary(Msambazaji* p) {')
if ins_pos < 0:
    ins_pos = c.find('\n// ==========================================================\n// EXPRESSION PARSER')
if ins_pos >= 0:
    c = c[:ins_pos] + '\n' + added + c[ins_pos:]

# Forward declarations - add ALL needed decls
fw_line = 'N32 changanua_taarifa(Msambazaji* p);\n'
fw_pos = c.find(fw_line)
if fw_pos >= 0:
    decls = ''
    for name in ['kama', 'wakati', 'chagua', 'kwa', 'tangazo', 'ndani']:
        decls += f'N32 changanua_taarifa_{name}(Msambazaji* p);\n'
    if 'ndani_kubwa' in added: decls += 'N32 changanua_ndani_kubwa(Msambazaji* p);\n'
    if 'kwa_malizia' in added: decls += 'N32 changanua_kwa_malizia(Msambazaji* p, N32 init, N32 cond, N32 step, N32 body);\n'
    c = c[:fw_pos+len(fw_line)] + decls + c[fw_pos+len(fw_line):]

# Add kazi + primary decls
fw_kazi = c.find('N32 changanua_kazi(Msambazaji* p);\n')
if fw_kazi >= 0:
    decls = 'N32 changanua_kazi_vigezo(Msambazaji* p);\nN32 changanua_kazi_mwili(Msambazaji* p);\n'
    c = c[:fw_kazi+len('N32 changanua_kazi(Msambazaji* p);\n')] + decls + c[fw_kazi+len('N32 changanua_kazi(Msambazaji* p);\n'):]

with open('msingi/msambazaji.swa', 'w') as f: f.write(c)
print('\nAll splits applied!')
