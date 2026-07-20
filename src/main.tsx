import { useEffect, useMemo, useRef, useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { demoCatalog, type CatalogEntry } from "./catalog";
import "./styles.css";

const copy = {
  en: { placeholder: "Search monsters, cards, items, maps…", empty: "No matching entries", pin: "Pin card", pinned: "Pinned", source: "Official source", refresh: "Demo catalog · official sync comes next", settings: "Settings", close: "Close", label: "SpiritVale field guide" },
  th: { placeholder: "ค้นหามอนสเตอร์ การ์ด ไอเทม แผนที่…", empty: "ไม่พบข้อมูลที่ตรงกัน", pin: "ปักหมุด", pinned: "ปักหมุดแล้ว", source: "แหล่งข้อมูลทางการ", refresh: "แค็ตตาล็อกตัวอย่าง · การซิงก์ทางการเร็ว ๆ นี้", settings: "การตั้งค่า", close: "ปิด", label: "คู่มือ SpiritVale" }
};

function score(entry: CatalogEntry, query: string) {
  const haystack = [entry.name, entry.kind, entry.subtitle, entry.summary, ...entry.tags].join(" ").toLowerCase();
  return query.split(/\s+/).filter(Boolean).every((term) => haystack.includes(term.toLowerCase()));
}

function App() {
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState<CatalogEntry>(demoCatalog[0]);
  const [pinned, setPinned] = useState(false);
  const [language, setLanguage] = useState<"en" | "th">("en");
  const inputRef = useRef<HTMLInputElement>(null);
  const text = copy[language];
  const results = useMemo(() => demoCatalog.filter((entry) => score(entry, query)), [query]);

  useEffect(() => { inputRef.current?.focus(); }, []);
  useEffect(() => { if (results[0] && !results.some((entry) => entry.id === selected.id)) setSelected(results[0]); }, [results, selected.id]);

  return <main className="overlay-shell">
    <div className="grain" aria-hidden="true" />
    <header className="masthead">
      <div className="brand"><span className="sigil">✦</span><div><strong>SPIRITVALE</strong><small>{text.label}</small></div></div>
      <div className="header-actions"><button className="language" onClick={() => setLanguage(language === "en" ? "th" : "en")}>{language === "en" ? "ไทย" : "EN"}</button><button className="icon-button" aria-label={text.settings}>⚙</button><button className="icon-button" aria-label={text.close} onClick={() => getCurrentWindow().hide()}>×</button></div>
    </header>
    <section className="search-zone">
      <span className="search-mark">⌕</span><input ref={inputRef} value={query} onChange={(event) => setQuery(event.target.value)} placeholder={text.placeholder} />
      <kbd>ALT&nbsp;+&nbsp;E</kbd>
    </section>
    <section className="content-grid">
      <aside className="result-list" aria-label="Search results">
        {results.map((entry) => <button key={entry.id} onClick={() => { setSelected(entry); setPinned(false); }} className={`result ${selected.id === entry.id ? "active" : ""}`}>
          <span className={`kind kind-${entry.kind.toLowerCase()}`}>{entry.kind}</span><span className="result-copy"><strong>{entry.name}</strong><small>{entry.subtitle}</small></span><span className="arrow">›</span>
        </button>)}
        {!results.length && <p className="empty">{text.empty}</p>}
      </aside>
      <article className="detail-card">
        <div className="detail-topline"><span className={`kind kind-${selected.kind.toLowerCase()}`}>{selected.kind}</span><span className="demo-chip">DEMO</span></div>
        <h1>{selected.name}</h1><p className="subtitle">{selected.subtitle}</p><p className="summary">{selected.summary}</p>
        <dl>{selected.fields.map((field) => <div key={field.label}><dt>{field.label}</dt><dd>{field.value}</dd></div>)}</dl>
        <div className="detail-actions"><button className={`pin-button ${pinned ? "is-pinned" : ""}`} onClick={() => setPinned(!pinned)}>{pinned ? "✦ " + text.pinned : "◇ " + text.pin}</button><button className="source-button" onClick={() => openUrl(selected.sourceUrl)}>{text.source} ↗</button></div>
      </article>
    </section>
    <footer><span className="pulse" />{text.refresh}<span className="divider">·</span><span>v0.1.0</span></footer>
  </main>;
}

export default App;
