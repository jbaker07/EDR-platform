/* app.js — hardened + with health + incidents + playbooks (safe if elements missing) */

(() => {
  "use strict";

  // ---------- DOM ----------
  const rowsEl      = document.getElementById('rows');
  const detailEl    = document.getElementById('detail');
  const sevEl       = document.getElementById('sevFilter');
  const searchEl    = document.getElementById('search');

  // Header stats (all optional)
  const statEvents  = document.getElementById('statEvents');
  const statAlerts  = document.getElementById('statAlerts');
  const statNodes   = document.getElementById('statNodes');
  const statEdges   = document.getElementById('statEdges');
  const statDrops   = document.getElementById('statDrops');
  const publishRibbonEl = document.getElementById('publishRibbon');
  const publishTargetsEl = document.getElementById('publishTargets');

  // Tabs / screens (optional)
  const screenDash  = document.getElementById('screenDash');
  const screenInt   = document.getElementById('screenIntegrations');
  const tabDash     = document.getElementById('tabDash');
  const tabInt      = document.getElementById('tabIntegrations');

  // Integrations pane (optional)
  const addDlg      = document.getElementById('addDlg');
  const vendorSel   = document.getElementById('vendorSel');
  const credFields  = document.getElementById('credFields');
  const tilesGrid   = document.getElementById('tiles');
  const btnAdd      = document.getElementById('btnAdd');
  const saveIntegrationBtn = document.getElementById('saveIntegration');

  // Explain drawer (optional)
  const drawer      = document.getElementById('drawer');
  const exCloseBtn  = document.getElementById('ex_close');

  // Timeline canvas (optional)
  const timelineCanvas = document.getElementById('timeline');

  // Playbooks UI (optional; renders only if present)
  const pbFeedEl    = document.getElementById('pb-feed');
  const pbHitsEl    = document.getElementById('pb-hits');
  const pbPendingEl = document.getElementById('pb-pending');

  // Incidents (DecisionEngine rollups) — optional UI
  const incWrap     = document.getElementById('incidents');
  const incListEl   = document.getElementById('incList');

  // ---------- State ----------
  const MAX_ALERTS = 2000;
  let alerts   = [];
  let eventsIn = 0;
  let catalog  = { vendors: [] };
  const incidents = new Map(); // id -> incident json

  // ---------- Helpers ----------
  const nowSec = () => Math.floor(Date.now() / 1000);
  const setText = (el, s) => { if (el) el.textContent = s; };
  const safeAdd = (el, child) => { if (el && child) el.appendChild(child); };

  function parseTs(x) {
    if (x == null) return nowSec();
    if (typeof x === 'number') return x > 2_000_000_000 ? Math.floor(x/1000) : x; // ms→s
    const s = String(x);
    const t = Date.parse(s);
    if (!Number.isNaN(t)) return Math.floor(t/1000);
    const n = Number(s);
    if (!Number.isNaN(n)) return n > 2_000_000_000 ? Math.floor(n/1000) : n;
    return nowSec();
  }

  const tsOf = (a) => parseTs(a?.ts ?? a?.time ?? a?.timestamp);
  const fmt  = (ts) => new Date(Number(ts) * 1000).toLocaleString();

  function severityCategory(a) {
    const s = a?.severity;
    if (typeof s === 'number') {
      if (s >= 0.80) return 'high';
      if (s >= 0.55) return 'medium';
      if (s > 0)     return 'low';
      return 'none';
    }
    return (String(s || '').toLowerCase());
  }
  const sevOf  = (a) => severityCategory(a);
  const riskOf = (a) => Number(a?.risk ?? a?.score ?? 0);

  function extractText(a) {
    const exe  = a?.event?.exe ?? a?.event?.binary_path ?? a?.event?.category ?? '';
    const cmd  = a?.event?.cmdline ?? a?.event?.command_line ?? '';
    const tags = (Array.isArray(a?.findings) ? a.findings : [])
      .map(f => (f && (f.label || f.rule || f.source)) || '')
      .join(' ');
    return { exe, cmd, tags };
  }

  function badgeClass(a) {
    const s = severityCategory(a);
    if (s === 'high')   return 'bg-rose-700/80';
    if (s === 'medium') return 'bg-amber-600/80';
    if (s === 'low')    return 'bg-emerald-700/80';
    return 'bg-slate-700/80';
  }

  function shortWhy(a) {
    if (!a) return '';
    if (a.explanation && (a.explanation.summary || a.explanation.short)) {
      return a.explanation.summary || a.explanation.short;
    }
    const sev = (severityCategory(a) || 'none').toUpperCase();
    const f = Array.isArray(a.findings) ? a.findings.slice() : [];
    f.sort((x, y) => (y?.score || 0) - (x?.score || 0));
    const top = f[0] || {};
    const technique =
      a.technique ||
      (top.label && /^T\d{4}(\.\d{3})?$/.test(top.label) ? top.label : null);
    const who = (top.source ? `${top.source}${top.rule ? ':'+top.rule : ''}` :
                (top.rule || top.id || top.label || null));
    const exe = a?.event?.exe || a?.event?.binary_path || a?.event?.category || '';
    const cmd = a?.event?.cmdline || a?.event?.command_line || '';
    const parts = [
      sev + (technique ? ` · ${technique}` : ''),
      who ? `by ${who}` : '',
      exe,
      cmd,
    ].filter(Boolean);
    const line = parts.join(' — ');
    return line.length > 240 ? (line.slice(0, 237) + '…') : line;
  }

  function synthIdFor(a) {
    const key = `${tsOf(a)}${a?.event?.exe || a?.event?.binary_path || '-'}${a?.event?.cmdline || a?.event?.command_line || '-'}`;
    let h = 0; for (let i = 0; i < key.length; i++) h = ((h << 5) - h) + key.charCodeAt(i) | 0;
    return String(h >>> 0);
  }

  function el(tag, cls, text) {
    const e = document.createElement(tag);
    if (cls) e.className = cls;
    if (text != null) e.textContent = String(text);
    return e;
  }

  const queryTemplates = {
    opensearch: (cgrp) => `edr.correlation_group_id:"${cgrp}" OR correlation_group_id:"${cgrp}"`,
    splunk:     (cgrp) => `index=edr_correlated_v2 correlation_group_id="${cgrp}" OR Cgrp_s="${cgrp}"`,
    sentinel:   (cgrp) => `EDR_CorrelatedV2_CL | where Cgrp_s == "${cgrp}"`
  };

  const openCopyMenus = new Set();

  function truncateId(id) {
    if (!id) return '—';
    return id.length > 8 ? `${id.slice(0, 4)}…${id.slice(-2)}` : id;
  }

  function buildSequenceLane(multistage = [], activeIdx = 0) {
    if (!multistage.length) return el('span', 'text-slate-500', '—');
    const wrap = el('div', 'flex gap-0.5 items-center');
    multistage.forEach((stage, idx) => {
      const block = el('span', 'text-xs', '▮');
      if (idx === activeIdx) {
        block.classList.add('text-sky-300');
      } else {
        block.classList.add('text-slate-500');
      }
      block.title = `${stage.stage} · score ${stage.score ?? 0}`;
      wrap.appendChild(block);
    });
    return wrap;
  }

  function renderSinks(sinks = {}) {
    const wrap = el('div', 'flex gap-1 mt-1');
    const defs = [
      { key: 'opensearch', label: 'OSD', tip: 'OpenSearch index' },
      { key: 'splunk', label: 'SPL', tip: 'Splunk HEC event' },
      { key: 'sentinel', label: 'SENT', tip: 'Microsoft Sentinel custom table' },
      { key: 'thehive', label: 'HIVE', tip: 'TheHive alert' }
    ];
    defs.forEach((def) => {
      if (sinks[def.key] == null) return;
      const pill = el('span', 'px-1.5 py-0.5 rounded text-[10px] border border-slate-700/60');
      pill.textContent = def.label;
      pill.title = def.tip;
      if (sinks[def.key]) {
        pill.classList.add('bg-emerald-600/40', 'text-emerald-200');
      } else {
        pill.classList.add('bg-slate-800/40', 'text-slate-400');
      }
      wrap.appendChild(pill);
    });
    if (!wrap.children.length) {
      wrap.appendChild(el('span', 'text-[11px] text-slate-500', 'no sinks'));
    }
    return wrap;
  }

  function copyQuery(target, cgrp) {
    const builder = queryTemplates[target];
    if (!builder) return;
    const query = builder(cgrp);
    if (navigator.clipboard?.writeText) {
      navigator.clipboard.writeText(query).catch(() => {
        window.prompt('Copy query', query); // eslint-disable-line no-alert
      });
    } else {
      window.prompt('Copy query', query); // eslint-disable-line no-alert
    }
  }

  function closeAllCopyMenus() {
    openCopyMenus.forEach((menu) => {
      menu.classList.add('hidden');
    });
    openCopyMenus.clear();
  }

  function createCopyMenu(cgrp) {
    const wrap = el('div', 'relative inline-block text-xs');
    if (!cgrp) {
      wrap.appendChild(el('span', 'text-slate-500', '—'));
      return wrap;
    }
    const toggle = el('button', 'px-2 py-1 rounded bg-slate-800 hover:bg-slate-700 border border-slate-700', 'Copy');
    const menu = el('div', 'hidden absolute right-0 mt-1 min-w-[120px] rounded border border-slate-700 bg-slate-900 shadow-lg z-10');
    ['opensearch', 'splunk', 'sentinel'].forEach((target) => {
      const item = el('button', 'block w-full text-left px-3 py-1 hover:bg-slate-800', target);
      item.onclick = (evt) => {
        evt.stopPropagation();
        copyQuery(target, cgrp);
        closeAllCopyMenus();
      };
      menu.appendChild(item);
    });
    toggle.onclick = (evt) => {
      evt.stopPropagation();
      if (menu.classList.contains('hidden')) {
        closeAllCopyMenus();
        menu.classList.remove('hidden');
        openCopyMenus.add(menu);
        document.addEventListener('click', closeAllCopyMenus, { once: true });
      } else {
        menu.classList.add('hidden');
        openCopyMenus.delete(menu);
      }
    };
    wrap.appendChild(toggle);
    wrap.appendChild(menu);
    return wrap;
  }

  function renderInlineEdges(edges) {
    const graphDiv = document.getElementById('ex_graph');
    if (!graphDiv) return;
    if (!edges || !edges.length) {
      graphDiv.innerHTML = '<div class="text-xs text-slate-500">Select an alert to view correlation edges.</div>';
      return;
    }
    const list = document.createElement('ul');
    list.className = 'text-xs space-y-1';
    edges.forEach((edge) => {
      const item = document.createElement('li');
      item.textContent = `${edge.kind || 'edge'} ${edge.src || '?'} → ${edge.dst || '?'}`;
      item.title = edge.vendor_source || '';
      list.appendChild(item);
    });
    graphDiv.innerHTML = '';
    graphDiv.appendChild(list);
  }

  // ---------- Tabs ----------
  function activateTab(which) {
    if (!screenDash || !screenInt || !tabDash || !tabInt) return;
    const onDash = which === 'dash';
    screenDash.classList.toggle('hidden', !onDash);
    screenInt.classList.toggle('hidden', onDash);
    tabDash.className = onDash
      ? 'px-3 py-1 rounded bg-sky-600 text-white'
      : 'px-3 py-1 rounded bg-slate-800 text-slate-200';
    tabInt.className = !onDash
      ? 'px-3 py-1 rounded bg-sky-600 text-white'
      : 'px-3 py-1 rounded bg-slate-800 text-slate-200';
    if (!onDash) loadIntegrations();
  }
  if (tabDash) tabDash.onclick = () => activateTab('dash');
  if (tabInt)  tabInt.onclick  = () => activateTab('int');
  activateTab('dash');

  // ---------- SSE Alerts ----------
  const es = new EventSource('/alerts');
  es.onmessage = (ev) => {
    try {
      const a = JSON.parse(ev.data);
      alerts.push(a);
      if (alerts.length > MAX_ALERTS) alerts.splice(0, alerts.length - MAX_ALERTS);
      setText(statAlerts, `alerts: ${alerts.length}`);
      renderTable();
      pushTimelinePoint(a);
    } catch { /* ignore */ }
  };
  es.onerror = () => { /* browser auto-retries SSE */ };

  // ---------- Incidents SSE (DecisionEngine rollups) ----------
  if (incListEl) {
    const esInc = new EventSource('/incidents/stream');
    esInc.onmessage = (ev) => {
      try {
        const it = JSON.parse(ev.data);
        if (it && it.id) {
          incidents.set(it.id, it);
          renderIncidents();
        }
      } catch {}
    };
  }

  function renderIncidents() {
    if (!incListEl) return;
    const items = Array.from(incidents.values())
      .sort((a,b) => (b.last_ts||0) - (a.last_ts||0))
      .slice(0, 40);
    incListEl.textContent = '';
    if (!items.length) {
      const p = incWrap?.querySelector('.text-slate-400');
      if (p) p.classList.remove('hidden');
      return;
    } else {
      const p = incWrap?.querySelector('.text-slate-400');
      if (p) p.classList.add('hidden');
    }
    for (const x of items) {
      const li = el('li','py-2 flex items-start justify-between gap-2');
      const left = el('div','min-w-0');
      const sevColor =
        x.severity_max === 'high' ? 'bg-rose-700/80' :
        x.severity_max === 'medium' ? 'bg-amber-600/80' :
        x.severity_max === 'low' ? 'bg-emerald-700/80' : 'bg-slate-700/80';
      const sev = el('span',`inline-block px-2 py-0.5 rounded text-xs mr-2 ${sevColor}`, (x.severity_max || 'none'));
      const head = el('div','font-medium truncate');
      head.appendChild(sev);
      head.appendChild(document.createTextNode(`${x.exe || '-'} · ${x.primary_technique || '-'}`));
      const sub = el('div','text-xs text-slate-400 mt-0.5',
        `alerts: ${x.alerts_count ?? 0} · risk_max: ${(x.risk_max ?? 0).toFixed(2)} · rgcn: ${(x.rgcn_score ?? 0).toFixed(2)} · ${new Date((x.last_ts||0)*1000).toLocaleString()}`);
      left.appendChild(head); left.appendChild(sub);
      const techs = Array.isArray(x.techniques) ? x.techniques.slice(0,6) : [];
      const chips = el('div','mt-1 flex flex-wrap gap-1 text-[11px]');
      for (const t of techs) chips.appendChild(el('span','px-1.5 py-0.5 rounded bg-slate-800', t));
      left.appendChild(chips);
      li.appendChild(left);
      incListEl.appendChild(li);
    }
  }

  // ---------- Metrics poll ----------
  async function pollMetrics() {
    try {
      const r = await fetch('/metrics');
      if (!r.ok) return;
      const m = await r.json();
      eventsIn = m.events_in || 0;
      setText(statEvents, `events: ${eventsIn}`);
      setText(statNodes,  `nodes: ${m.nodes_count ?? 0}`);
      setText(statEdges,  `edges: ${m.edges_count ?? 0}`);
      setText(statDrops,  `bpf_drops: ${m.bpf_drops_total ?? 0}`);
    } catch { /* ignore */ }
  }
  setInterval(pollMetrics, 5000); pollMetrics();

  async function refreshPublishStatus() {
    if (!publishTargetsEl) return;
    try {
      const res = await fetch('/api/connectors/status', { cache: 'no-store' });
      if (!res.ok) throw new Error(`${res.status}`);
      const body = await res.json();
      const list = Array.isArray(body.connectors) ? body.connectors : [];
      const enabled = list.filter((c) => c.enabled);
      if (!enabled.length) {
        publishTargetsEl.textContent = 'none';
        publishRibbonEl?.classList.add('opacity-60');
        publishRibbonEl?.classList.remove('text-rose-300');
      } else {
        publishTargetsEl.textContent = enabled
          .map((c) => c.name || c.id || c.type)
          .join(', ');
        publishRibbonEl?.classList.remove('opacity-60');
        publishRibbonEl?.classList.remove('text-rose-300');
      }
    } catch (err) {
      publishTargetsEl.textContent = 'offline';
      publishRibbonEl?.classList.remove('opacity-60');
      publishRibbonEl?.classList.add('text-rose-300');
    }
  }

  setInterval(refreshPublishStatus, 30000);
  refreshPublishStatus();

  // ---------- Alerts table ----------
  if (sevEl)   sevEl.onchange   = renderTable;
  if (searchEl) searchEl.oninput = renderTable;

  function renderTable() {
    if (!rowsEl) return;
    const sevFilter = (sevEl && sevEl.value || '').toLowerCase();
    const q = (searchEl && searchEl.value || '').toLowerCase();
    rowsEl.textContent = ''; // clear safely

    // newest first
    for (let i = alerts.length - 1; i >= 0; i--) {
      const a = alerts[i];
      if (sevFilter && sevOf(a) !== sevFilter) continue;

      const { exe, cmd, tags } = extractText(a);
      const hay = (exe + ' ' + cmd + ' ' + tags).toLowerCase();
      if (q && !hay.includes(q)) continue;

      const tr = el('tr', 'hover:bg-slate-900 cursor-pointer');
      tr.title = shortWhy(a);
      tr.onclick = () => selectAlert(a);

      const findings = (a.findings || [])
        .map(f => `${f?.source ?? 'detector'}:${Number(f?.score || 0).toFixed(2)}${f?.label ? `(${f.label})` : ''}`)
        .join(', ');
      const risk = riskOf(a).toFixed(2);
      const ts = tsOf(a);

      const tdTime = el('td', 'px-3 py-2', fmt(ts));

      const tdSev = el('td', 'px-3 py-2');
      const sevBadge = el('span', `px-2 py-1 rounded text-xs ${badgeClass(a)}`, severityCategory(a) || 'none');
      tdSev.appendChild(sevBadge);

      const tdRisk = el('td', 'px-3 py-2', risk);

      const tdEvent = el('td', 'px-3 py-2 truncate max-w-[520px]');
      const exeDiv  = el('div', 'text-slate-200', exe || '-');
      const cmdDiv  = el('div', 'text-slate-400', cmd || '');
      tdEvent.appendChild(exeDiv); tdEvent.appendChild(cmdDiv);

      const tdFind = el('td', 'px-3 py-2', findings);

      tr.appendChild(tdTime);
      tr.appendChild(tdSev);
      tr.appendChild(tdRisk);
      tr.appendChild(tdEvent);
      tr.appendChild(tdFind);
      rowsEl.appendChild(tr);
    }
  }

  // ---------- Timeline chart ----------
  const timelineData = { datasets: [{ label: 'alerts', data: [], pointRadius: 4 }] };
  let timelineChart = null;

  if (timelineCanvas && window.Chart) {
    const timelineCtx = timelineCanvas.getContext('2d');
    const hasTimeScale = !!(Chart?.registry?.getScale?.('time'));
    const xScale = hasTimeScale
      ? { type: 'time', time: { unit: 'minute' }, ticks: { color: '#cbd5e1' }, grid: { color: '#334155' } }
      : { type: 'linear', ticks: { color: '#cbd5e1' }, grid: { color: '#334155' } };

    timelineChart = new Chart(timelineCtx, {
      type: 'scatter',
      data: timelineData,
      options: {
        animation: false,
        plugins: {
          legend: { labels: { color: '#cbd5e1' } },
          tooltip: { callbacks: { label: (ctx) => shortWhy(ctx.raw?.__alert) } }
        },
        scales: {
          x: xScale,
          y: { display: false }
        },
        onClick: (_, elems) => {
          if (elems?.length) {
            const idx = elems[0].index;
            const obj = timelineData.datasets[0].data[idx].__alert;
            selectAlert(obj);
          }
        }
      }
    });
  }

  function pushTimelinePoint(a) {
    if (!timelineChart) return;
    const t = tsOf(a) * 1000;
    timelineData.datasets[0].data.push({ x: t, y: 1, __alert: a });
    if (timelineData.datasets[0].data.length > 400) timelineData.datasets[0].data.shift();
    timelineChart.update('none');
  }

  // ---------- Explain drawer ----------
  if (exCloseBtn) exCloseBtn.onclick = () => drawer && drawer.classList.add('hidden');

  async function openExplain(id) {
    try {
      const r = await fetch(`/alerts/${encodeURIComponent(id)}/explain`);
      if (!r.ok) return;
      const ex = await r.json();
      renderExplain(ex);
      if (drawer) drawer.classList.remove('hidden');
    } catch { /* ignore */ }
  }

  function renderExplain(ex) {
    if (!ex) return;
    const H = (id, v) => { const n = document.getElementById(id); if (n) n.textContent = v; };

    // headline + scores
    H('ex_head', ex.headline || 'Explain');
    H('ex_risk', Number(ex.risk || 0).toFixed(2));
    H('ex_rgcn', Number(ex.rgcn_score || 0).toFixed(2));

    // entities
    H('ex_entities', JSON.stringify(ex.entities || {}, null, 2));

    // findings
    const whyUl = document.getElementById('ex_why');
    if (whyUl) {
      whyUl.textContent = '';
      (ex.why || []).forEach(w => {
        const li  = el('li', null,
          `${w?.source || 'detector'} · ${Number(w?.score || 0).toFixed(2)}${w?.label ? ` · ${w.label}` : ''}`
        );
        whyUl.appendChild(li);
      });
    }
    H('ex_anoms', (ex.anomalies || []).join(', ') || '—');

    // graph
    const g = ex.graph || { nodes: [], edges: [], links: [] };
    const edges = Array.isArray(g.edges) && g.edges.length ? g.edges : (g.links || []);
    const graphDiv = document.getElementById('ex_graph');
    if (graphDiv) {
      graphDiv.innerHTML = '';
      if ((g.nodes || []).length) {
        ensureCytoscape().then(() => {
          graphDiv.innerHTML = '';
          cytoscape({
            container: graphDiv,
            elements: {
              nodes: g.nodes.map(n => ({ data: { id: n.id, label: (n.binary_path || n.id) } })),
              edges: edges.map(e => ({ data: { source: e.source, target: e.target } }))
            },
            style: [
              { selector: 'node', style: { 'background-color': '#22d3ee', 'label': 'data(label)', 'font-size': '10px', 'color': '#0f172a', 'shape': 'round-rectangle', 'padding': '5px' } },
              { selector: 'edge', style: { 'line-color': '#64748b', 'target-arrow-color': '#64748b', 'target-arrow-shape': 'triangle' } }
            ],
            layout: { name: 'breadthfirst', directed: true, padding: 8 }
          });
        });
      }
    }

    // timeline (explain window)
    const tlWrap = document.getElementById('ex_timeline');
    if (tlWrap) {
      tlWrap.textContent = '';
      const frag = document.createDocumentFragment();
      (ex.timeline || []).slice(0, 120).forEach(e => {
        const line = el('div', null, `${new Date(parseTs(e.ts ?? e.time ?? e.timestamp) * 1000).toLocaleTimeString()} — pid ${e.pid ?? '—'} → ${e.exe || ''} `);
        const span = el('span', 'text-slate-500', e.cmd || '');
        line.appendChild(span);
        frag.appendChild(line);
      });
      tlWrap.appendChild(frag);
      if (!(ex.timeline || []).length) tlWrap.textContent = '—';
    }

    // enrichment
    H('ex_enrich', JSON.stringify(ex.enrichment || {}, null, 2));

    // actions
    const actionsUl = document.getElementById('ex_actions');
    if (actionsUl) {
      actionsUl.textContent = '';
      (ex.recommendations || []).forEach(a => actionsUl.appendChild(el('li', null, a)));
    }
  }

  function selectAlert(a) {
    if (detailEl) detailEl.textContent = JSON.stringify(a, null, 2);
    const id = a.id || synthIdFor(a);
    openExplain(id);
  }

  // ---------- Integrations ----------
  async function loadCatalog() {
    try {
      const r = await fetch('/integrations/catalog');
      if (!r.ok) return;
      const cat = await r.json();
      catalog.vendors = Array.isArray(cat?.vendors) ? cat.vendors : (Array.isArray(cat?.items) ? cat.items : []);
      if (vendorSel) {
        vendorSel.innerHTML = (catalog.vendors || []).map(v => `<option value="${String(v.id)}">${String(v.name || v.id)}</option>`).join('');
        vendorSel.onchange = renderCredFields;
      }
      renderCredFields();
    } catch { /* ignore */ }
  }

  function renderCredFields() {
    if (!credFields || !vendorSel) return;
    const v = (catalog.vendors || []).find(x => String(x.id) === String(vendorSel.value));
    const fields = (v?.auth?.fields || v?.required_fields || []);
    credFields.textContent = '';
    const frag = document.createDocumentFragment();
    for (const f of fields) {
      const lab = el('label', 'block text-sm capitalize', String(f).replace('_',' '));
      const inp = el('input', 'w-full bg-slate-900 border border-slate-700 rounded px-2 py-1');
      inp.id = `field_${f}`;
      inp.placeholder = String(f);
      frag.appendChild(lab);
      frag.appendChild(inp);
    }
    credFields.appendChild(frag);
  }

  if (btnAdd) btnAdd.onclick = async () => { await loadCatalog(); addDlg?.showModal?.(); };

  if (saveIntegrationBtn) saveIntegrationBtn.onclick = async (e) => {
    e.preventDefault();
    if (!vendorSel) return;
    const id = vendorSel.value;
    const v  = (catalog.vendors || []).find(x => String(x.id) === String(id));
    const fields = (v?.auth?.fields || v?.required_fields || []);
    const creds = {};
    for (const f of fields) {
      const elv = document.getElementById(`field_${f}`);
      creds[f] = elv ? (elv.value || '') : '';
    }
    try {
      await fetch('/integrations', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ id, kind: id, name: v?.name || id, creds })
      });
    } catch {}
    addDlg?.close?.();
    loadIntegrations();
  };

  async function loadIntegrations() {
    try {
      const r = await fetch('/integrations');
      if (!r.ok) return;
      const data = await r.json();

      // normalize items
      let items = [];
      if (Array.isArray(data)) {
        if (data.length && data[0]?.cfg) {
          items = data.map(x => ({
            id: x.cfg.id,
            name: x.cfg.name,
            status: x.rt?.status || 'unknown',
            last_ok: x.cfg?.last_ok || null
          }));
        } else {
          items = data;
        }
      } else {
        items = data.items || [];
      }

      if (!tilesGrid) return;
      tilesGrid.textContent = '';
      const frag = document.createDocumentFragment();

      for (const it of items) {
        const card = el('div', 'rounded border border-slate-800 p-3 bg-slate-900/50');

        const row1 = el('div', 'flex items-center justify-between');
        row1.appendChild(el('div', 'font-medium', it.name || it.id));
        row1.appendChild(el('span', 'text-xs text-slate-400', it.status || 'unknown'));
        card.appendChild(row1);

        card.appendChild(el('div', 'text-xs text-slate-400 mt-1', `last ok: ${it.last_ok || '—'}`));

        const btnRow = el('div', 'mt-2 flex gap-2');
        const bGraph = el('button', 'btnGraph px-2 py-1 rounded bg-slate-800 text-xs', 'Graph');
        const bDel   = el('button', 'btnDel px-2 py-1 rounded bg-rose-700/80 text-xs', 'Remove');
        bGraph.dataset.id = it.id; bDel.dataset.id = it.id;
        btnRow.appendChild(bGraph); btnRow.appendChild(bDel);
        card.appendChild(btnRow);

        const wrap = el('div', 'mt-2 hidden'); wrap.id = `graph_${it.id}`;
        wrap.appendChild(el('div', 'text-xs text-slate-400 mb-1', 'Endpoint R-GCN score (approx.): '));
        const rg = el('span'); rg.id = `rg_${it.id}`; wrap.firstChild.appendChild(rg);
        const cy = el('div'); cy.id = `cy_${it.id}`; cy.style.height = '180px';
        wrap.appendChild(cy);
        card.appendChild(wrap);

        frag.appendChild(card);
      }
      tilesGrid.appendChild(frag);

      for (const b of tilesGrid.querySelectorAll('.btnDel')) {
        b.onclick = async () => {
          try { await fetch(`/integrations/${encodeURIComponent(b.dataset.id)}`, { method: 'DELETE' }); } catch {}
          loadIntegrations();
        };
      }

      for (const b of tilesGrid.querySelectorAll('.btnGraph')) {
        b.onclick = async () => {
          const id = b.dataset.id;
          try {
            const r = await fetch(`/graph/${encodeURIComponent(id)}?window_sec=900`);
            if (!r.ok) return;
            const g = await r.json();
            const rgEl = document.getElementById(`rg_${id}`);
            if (rgEl) rgEl.textContent = Number(g.rgcn_score || 0).toFixed(2);
            const wrap = document.getElementById(`graph_${id}`); if (wrap) wrap.classList.remove('hidden');

            await ensureCytoscape();
            const container = document.getElementById(`cy_${id}`);
            if (container) {
              container.innerHTML = ''; // idempotent
              const edges = Array.isArray(g.edges) && g.edges.length ? g.edges : (g.links || []);
              cytoscape({
                container,
                elements: {
                  nodes: (g.nodes || []).map(n => ({ data: { id: n.id, label: (n.binary_path || n.id) } })),
                  edges: edges.map(e => ({ data: { source: e.source, target: e.target } }))
                },
                style: [
                  { selector: 'node', style: { 'background-color': '#22d3ee', 'label': 'data(label)', 'font-size': '10px', 'color': '#0f172a', 'shape': 'round-rectangle', 'padding': '5px' } },
                  { selector: 'edge', style: { 'line-color': '#64748b', 'target-arrow-color': '#64748b', 'target-arrow-shape': 'triangle' } }
                ],
                layout: { name: 'breadthfirst', directed: true, padding: 8 }
              });
            }
          } catch { /* ignore */ }
        };
      }
    } catch { /* ignore */ }
  }
  loadIntegrations();

  // ---------- Playbooks wiring (SSE + snapshots; optional) ----------
  function renderPbItem(item) {
    const ts = (item?.ts || item?.time || 0) * 1000;
    const when = ts ? new Date(ts).toLocaleString() : '';
    const title = item?.playbook_name || item?.playbook_id || 'playbook';
    const sev = (item?.severity || '').toString().toUpperCase();
    const rationale = Array.isArray(item?.rationale) ? item.rationale.join(' | ') : '';
    const tags = Array.isArray(item?.tags) ? item.tags.join(', ') : '';

    const li = el('li', 'text-sm');
    if (when) li.appendChild(el('span', 'text-slate-400 mr-2', `[${when}]`));
    li.appendChild(el('span', 'font-medium mr-2', title));
    if (sev) li.appendChild(el('span', 'px-1 py-0.5 rounded bg-slate-700/70 text-xs mr-2', sev));
    if (tags) li.appendChild(el('span', 'text-slate-400 mr-2', tags));
    if (rationale) li.appendChild(el('span', 'text-slate-300', rationale));
    return li;
  }

  function appendPbFeed(item) {
    if (!pbFeedEl) return;
    const li = renderPbItem(item);
    pbFeedEl.prepend(li);
    while (pbFeedEl.children.length > 200) pbFeedEl.removeChild(pbFeedEl.lastChild);
  }

  async function loadPlaybooksSnapshots() {
    try {
      const [hitsR, pendR] = await Promise.all([
        fetch('/playbooks/hits'),
        fetch('/playbooks/pending')
      ]);
      if (hitsR.ok) {
        const hits = await hitsR.json();
        if (pbHitsEl) {
          pbHitsEl.textContent = '';
          (Array.isArray(hits) ? hits : []).slice(-200).forEach(h => safeAdd(pbHitsEl, renderPbItem(h)));
        }
      }
      if (pendR.ok) {
        const pending = await pendR.json();
        if (pbPendingEl) {
          pbPendingEl.textContent = '';
          (Array.isArray(pending) ? pending : []).slice(-200).forEach(p => safeAdd(pbPendingEl, renderPbItem(p)));
        }
      }
    } catch { /* ignore */ }
  }

  (function mountPlaybooksSSE() {
    if (!pbFeedEl && !pbHitsEl && !pbPendingEl) return; // nothing to render
    try {
      const esPB = new EventSource('/playbooks/stream');
      esPB.onmessage = (evt) => {
        try {
          const msg = JSON.parse(evt.data);
          const item = msg.item || msg; // tolerate either {kind,item} or direct item
          appendPbFeed(item);
        } catch (e) {
          console.warn('PB stream parse error', e);
        }
      };
      esPB.onerror = (e) => console.warn('PB stream error', e);
      loadPlaybooksSnapshots();
      setInterval(loadPlaybooksSnapshots, 30000);
    } catch (e) {
      console.warn('PB SSE init failed', e);
    }
  })();

  // ---------- Lazy loader for Cytoscape ----------
  function ensureCytoscape() {
    if (window.cytoscape) return Promise.resolve();
    return new Promise((resolve, reject) => {
      const s = document.createElement('script');
      s.src = 'https://cdn.jsdelivr.net/npm/cytoscape@3.26.0/dist/cytoscape.min.js';
      s.onload = () => resolve();
      s.onerror = reject;
      document.head.appendChild(s);
    });
  }
})();

// ===================== Integration Filters & OpenSearch helpers =====================
(function () {
  // Point this to your OpenSearch or to your backend proxy (recommended in browsers).
  // If you expose a proxy like /api/os/*, set OS_BASE = "" and call fetch("/api/os/_search", ...).
  const OS_BASE = "http://127.0.0.1:9200";
  const INDEX   = "events-all-*"; // single, normalized stream (deduped, no overlap)

  function minutesAgoISO(mins) {
    const d = new Date(Date.now() - mins * 60 * 1000);
    return d.toISOString();
  }
  function selectedSources() {
    return Array.from(document.querySelectorAll('input[name="source"]:checked')).map(cb => cb.value);
  }
  function selectedMinutes() {
    const v = parseInt(document.getElementById("minutes")?.value || "60", 10);
    return isFinite(v) && v > 0 ? v : 60;
  }

  // Build a reusable bool.filter array for any OpenSearch query
  function buildMustFilters(extra = []) {
    const mins = selectedMinutes();
    const sources = selectedSources();

    const base = [
      { range: { "@timestamp": { gte: minutesAgoISO(mins) } } }
    ];
    if (sources.length) {
      base.push({ terms: { "fields.source": sources } });
    }
    return base.concat(extra || []);
  }

  // Convenience: wrap POST to _search
  async function osSearch(body, path = `/${INDEX}/_search`) {
    const res = await fetch(`${OS_BASE}${path}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body)
    });
    if (!res.ok) {
      const text = await res.text().catch(() => "");
      throw new Error(`${res.status} ${res.statusText} ${text}`);
    }
    return res.json();
  }

  // Optional: tiny demo queries you can call from existing code
  async function searchRecentSample(limit = 25) {
    const body = {
      size: limit,
      sort: [{ "@timestamp": { order: "desc" } }],
      query: { bool: { filter: buildMustFilters() } },
      _source: ["@timestamp","fields.source","rule.name","source.ip","destination.ip","event.severity","risk.score"]
    };
    return osSearch(body);
  }
  async function topAlertNames(size = 10) {
    const body = {
      size: 0,
      query: { bool: { filter: buildMustFilters() } },
      aggs: { top_names: { terms: { field: "rule.name.keyword", size, missing: "(missing)" } } }
    };
    return osSearch(body);
  }

  function setStatus(msg) {
    const el = document.getElementById("osStatus");
    if (el) el.textContent = msg || "";
  }

  async function refreshHookIfPresent() {
    // If your existing app exposes a global refresh, call it so the new filters apply.
    // Otherwise, we don't do anything intrusive here.
    try {
      if (typeof window.refreshData === "function") {
        setStatus("updating…");
        await window.refreshData(); // your page’s existing fetch/render
        setStatus(`updated ${new Date().toLocaleTimeString()}`);
      }
    } catch (e) {
      console.error(e);
      setStatus(`error: ${e.message}`);
    }
  }

  // Wire up events
  function wireUI() {
    const btn = document.getElementById("refreshBtn");
    if (btn && !btn.dataset.wired) {
      btn.addEventListener("click", refreshHookIfPresent);
      btn.dataset.wired = "1";
    }
    // Optional: refresh when checkboxes/time change
    document.querySelectorAll('input[name="source"]').forEach(cb => {
      if (!cb.dataset.wired) {
        cb.addEventListener("change", refreshHookIfPresent);
        cb.dataset.wired = "1";
      }
    });
    const mins = document.getElementById("minutes");
    if (mins && !mins.dataset.wired) {
      mins.addEventListener("change", refreshHookIfPresent);
      mins.dataset.wired = "1";
    }
  }

  document.addEventListener("DOMContentLoaded", wireUI);

  // Expose a small, stable API for the rest of the app
  window.integrationFilters = {
    getIndexPattern: () => INDEX,
    getSelectedSources: selectedSources,
    getSelectedMinutes: selectedMinutes,
    buildMustFilters,          // -> use inside your existing OpenSearch queries
    osSearch,                  // -> optional direct search helper
    searchRecentSample,        // -> optional demo
    topAlertNames,             // -> optional demo
  };
})();
