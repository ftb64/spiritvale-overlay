#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::HashMap, fs, time::{SystemTime, UNIX_EPOCH}};
use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

const SOURCES: [(&str, &str, &str); 9] = [
  ("Artifacts", "artifacts", "artifacts"), ("Cards", "cards", "cards"),
  ("Consumables", "consumables", "consumables"), ("Gems", "gems", "gems"),
  ("Maps", "maps", "maps"), ("Materials", "materials", "materials"),
  ("Monsters", "monsters", "monsters"), ("Equipment", "equipment", "equipmentList"),
  ("Skills", "skills", "skills"),
];

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CatalogEntry { id: String, kind: String, name: String, subtitle: String, summary: String, tags: Vec<String>, source_url: String, image_url: Option<String>, aliases: String, fields: Vec<CatalogField> }
#[derive(Clone, Serialize, Deserialize)]
struct CatalogField { label: String, value: String }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogResponse { entries: Vec<CatalogEntry>, source: String, synced_at: u64 }
#[derive(Serialize, Deserialize)]
struct CachedCatalog { entries: Vec<CatalogEntry>, synced_at: u64 }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowPreferences { width: u32, height: u32 }

const WINDOW_SIZE_PRESETS: &[(u32, u32)] = &[(960, 540), (1280, 720), (1600, 900), (1920, 1080)];

fn now_epoch() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn cache_path(app: &AppHandle) -> Result<std::path::PathBuf, String> { let directory = app.path().app_data_dir().map_err(|error| error.to_string())?; fs::create_dir_all(&directory).map_err(|error| error.to_string())?; Ok(directory.join("database-cache.json")) }
fn window_preferences_path(app: &AppHandle) -> Result<std::path::PathBuf, String> { let directory = app.path().app_data_dir().map_err(|error| error.to_string())?; fs::create_dir_all(&directory).map_err(|error| error.to_string())?; Ok(directory.join("window-preferences.json")) }
fn save_window_preferences(app: &AppHandle, preferences: &WindowPreferences) -> Result<(), String> { let path = window_preferences_path(app)?; fs::write(path, serde_json::to_vec(preferences).map_err(|error| error.to_string())?).map_err(|error| error.to_string()) }
fn saved_window_preferences(app: &AppHandle) -> Option<WindowPreferences> { let path = window_preferences_path(app).ok()?; let bytes = fs::read(path).ok()?; serde_json::from_slice(&bytes).ok() }
fn is_window_size_preset(width: u32, height: u32) -> bool { WINDOW_SIZE_PRESETS.iter().any(|(preset_width, preset_height)| *preset_width == width && *preset_height == height) }
fn apply_window_size(window: &WebviewWindow, width: u32, height: u32) -> Result<(), String> { window.set_size(tauri::Size::Logical(tauri::LogicalSize::new(width as f64, height as f64))).map_err(|error| error.to_string()) }
fn clean_text(value: &str) -> String { Regex::new(r"<[^>]*>").expect("valid tag regex").replace_all(&value.replace("&lt;", "<").replace("&gt;", ">"), "").replace("&amp;", "&").replace("&#039;", "'").trim().to_string() }
fn text(object: &Map<String, Value>, keys: &[&str]) -> String { keys.iter().find_map(|key| object.get(*key).and_then(Value::as_str)).map(clean_text).unwrap_or_default() }
fn number(object: &Map<String, Value>, key: &str) -> String { object.get(key).and_then(Value::as_i64).map(|value| value.to_string()).unwrap_or_default() }
fn strings(object: &Map<String, Value>, keys: &[&str]) -> String { keys.iter().filter_map(|key| object.get(*key).and_then(Value::as_array)).map(|values| values.iter().filter_map(Value::as_str).map(clean_text).filter(|value| !value.is_empty()).collect::<Vec<_>>().join("\n")).filter(|value| !value.is_empty()).collect::<Vec<_>>().join("\n") }
fn maps(object: &Map<String, Value>) -> String {
  let mut locations = Vec::new();
  for value in object.get("maps").and_then(Value::as_array).into_iter().flatten() {
    let Some(map) = value.as_object() else { continue; };
    let name = text(map, &["name", "DisplayName", "Name"]);
    if name.is_empty() { continue; }
    let min_level = map.get("minLevel").and_then(Value::as_i64).unwrap_or_default();
    let max_level = map.get("maxLevel").and_then(Value::as_i64).unwrap_or_default();
    let location = if min_level > 0 && max_level > 0 {
      format!("{name} (Lv {min_level} - {max_level})")
    } else if min_level > 0 {
      format!("{name} (Lv {min_level}+)")
    } else {
      name
    };
    if !locations.contains(&location) { locations.push(location); }
  }
  locations.join("\n")
}
fn related(object: &Map<String, Value>) -> String { object.get("drops").and_then(Value::as_array).map(|values| values.iter().filter_map(|value| { let item = value.as_object()?; let nested = item.get("monster").and_then(Value::as_object).unwrap_or(item); let name = text(nested, &["name", "DisplayName", "Name"]); if name.is_empty() { return None; } let chance = item.get("chance").and_then(Value::as_f64); Some(chance.map(|value| format!("{name} {value}%")).unwrap_or(name)) }).collect::<Vec<_>>().join("\n")).unwrap_or_default() }
fn add_drop_source(index: &mut HashMap<String, Vec<String>>, drop_type: &str, key: &str, source: &str) {
  let key = clean_text(key).to_ascii_lowercase();
  if key.is_empty() { return; }
  let values = index.entry(format!("{drop_type}:{key}")).or_default();
  if !values.contains(&source.to_string()) { values.push(source.to_string()); }
}
fn monster_drop_index(payload: &str) -> Result<HashMap<String, Vec<String>>, String> {
  let capture = Regex::new(r#"data-page=\"([^\"]+)\""#).expect("valid page regex").captures(payload).ok_or("SpiritVale Info did not provide monster drop data.")?;
  let page: Value = serde_json::from_str(&capture[1].replace("&quot;", "\"").replace("&amp;", "&")).map_err(|error| error.to_string())?;
  let records = page.pointer("/props/monsters").and_then(Value::as_array).ok_or("SpiritVale Info did not provide monster records.")?;
  let mut index = HashMap::new();
  for record in records {
    let Some(monster) = record.as_object() else { continue; };
    let name = text(monster, &["DisplayName", "name", "Name"]); if name.is_empty() { continue; }
    let level = number(monster, "Level");
    for value in monster.get("drops").and_then(Value::as_array).into_iter().flatten() {
      let Some(drop) = value.as_object() else { continue; };
      let drop_type = text(drop, &["type"]).to_ascii_lowercase();
      if drop_type != "card" && drop_type != "gem" { continue; }
      let chance = drop.get("chance").and_then(Value::as_f64).map(|value| format!(" · {value}%")).unwrap_or_default();
      let source = if level.is_empty() { format!("{name}{chance}") } else { format!("{name} (Lv {level}){chance}") };
      for key in [text(drop, &["slug"]), text(drop, &["id"]), text(drop, &["name"])] { add_drop_source(&mut index, &drop_type, &key, &source); }
    }
  }
  Ok(index)
}
fn add_monster_location(index: &mut HashMap<String, Vec<String>>, key: &str, location: &str) {
  let key = clean_text(key).to_ascii_lowercase();
  if key.is_empty() { return; }
  let locations = index.entry(key).or_default();
  if !locations.contains(&location.to_string()) { locations.push(location.to_string()); }
}
fn monster_location_index(payload: &str) -> Result<HashMap<String, Vec<String>>, String> {
  let capture = Regex::new(r#"data-page=\"([^\"]+)\""#).expect("valid page regex").captures(payload).ok_or("SpiritVale Info did not provide map location data.")?;
  let page: Value = serde_json::from_str(&capture[1].replace("&quot;", "\"").replace("&amp;", "&")).map_err(|error| error.to_string())?;
  let records = page.pointer("/props/maps").and_then(Value::as_array).ok_or("SpiritVale Info did not provide map records.")?;
  let mut index = HashMap::new();
  for record in records {
    let Some(map) = record.as_object() else { continue; };
    let location = text(map, &["DisplayName", "GameId", "name", "Name"]); if location.is_empty() { continue; }
    for name in map.get("MonsterPool").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str) { add_monster_location(&mut index, name, &location); }
    for value in map.get("monsters").and_then(Value::as_array).into_iter().flatten() {
      let Some(monster) = value.as_object() else { continue; };
      for key in [text(monster, &["Slug"]), text(monster, &["GameId"]), text(monster, &["DisplayName"])] { add_monster_location(&mut index, &key, &location); }
    }
  }
  Ok(index)
}
fn map_monsters(object: &Map<String, Value>) -> String {
  let mut monsters = Vec::new();
  for value in object.get("monsters").and_then(Value::as_array).into_iter().flatten() {
    let Some(monster) = value.as_object() else { continue; };
    let name = text(monster, &["DisplayName", "name", "Name", "GameId"]); if name.is_empty() { continue; }
    let level = number(monster, "Level");
    let element = text(monster, &["Element"]);
    let detail = [if level.is_empty() { String::new() } else { format!("Lv {level}") }, element].into_iter().filter(|value| !value.is_empty()).collect::<Vec<_>>().join(" · ");
    let value = if detail.is_empty() { name } else { format!("{detail} · {name}") };
    if !monsters.contains(&value) { monsters.push(value); }
  }
  monsters.join("\n")
}
fn display_decimal(value: f64) -> String {
  if value.fract() == 0.0 { format!("{}", value as i64) } else { value.to_string() }
}
fn skill_metric(object: &Map<String, Value>, key: &str, unit: &str) -> String {
  let Some(metric) = object.get("values").and_then(Value::as_object).and_then(|values| values.get(key)).and_then(Value::as_object) else { return String::new(); };
  let Some(base) = metric.get("base").and_then(Value::as_f64) else { return String::new(); };
  let level = metric.get("level").and_then(Value::as_f64).unwrap_or(0.0);
  let base = display_decimal(base);
  if level > 0.0 { format!("{base} + {} {unit} per level", display_decimal(level)) }
  else if level < -1.0 { format!("{base} - {} {unit} per level", display_decimal(-level)) }
  else { format!("{base} {unit}") }
}
fn parse_entries(payload: &str, property: &str, kind: &str, path: &str, dropped_by: &HashMap<String, Vec<String>>, monster_locations: &HashMap<String, Vec<String>>) -> Result<Vec<CatalogEntry>, String> {
  let capture = Regex::new(r#"data-page=\"([^\"]+)\""#).expect("valid page regex").captures(payload).ok_or("SpiritVale Info did not provide database data.")?;
  let page: Value = serde_json::from_str(&capture[1].replace("&quot;", "\"").replace("&amp;", "&")).map_err(|error| error.to_string())?;
  let records = page.pointer(&format!("/props/{property}")).and_then(Value::as_array).ok_or_else(|| format!("SpiritVale Info did not provide {kind} records."))?;
  Ok(records.iter().filter_map(|record| {
    let object = record.as_object()?;
    let name = text(object, &["DisplayName", "name", "Name", "GameId"]); if name.is_empty() { return None; }
    let slug = text(object, &["Slug", "slug"]); let aliases = text(object, &["GameId", "Id"]); let level = number(object, "Level"); let element = text(object, &["Element", "typeName", "slot", "Type"]);
    let is_boss = number(object, "IsBoss"); let subtitle = [if level.is_empty() { String::new() } else { format!("Level {level}") }, element.clone()].into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" · ");
    let summary = text(object, &["Description", "description"]);
    let mut fields = Vec::new();
    if kind == "Artifacts" {
      let drop_locations = maps(object); if !drop_locations.is_empty() { fields.push(CatalogField { label: "Drops".into(), value: drop_locations }); }
      let per_piece = strings(object, &["statsPerPiece"]); if !per_piece.is_empty() { fields.push(CatalogField { label: "Per piece".into(), value: per_piece }); }
      let full_set = strings(object, &["statsFullSet"]); if !full_set.is_empty() { fields.push(CatalogField { label: "Full set".into(), value: full_set }); }
      let per_refine = strings(object, &["statsPerRefine"]); if !per_refine.is_empty() { fields.push(CatalogField { label: "Per refine".into(), value: per_refine }); }
    } else if kind == "Maps" {
      let monsters = map_monsters(object); if !monsters.is_empty() { fields.push(CatalogField { label: "Monsters".into(), value: monsters }); }
    } else if kind == "Skills" {
      let skill_type = if object.get("isPassive").and_then(Value::as_bool).unwrap_or(false) { "Passive" } else { "Active" };
      fields.push(CatalogField { label: "Type".into(), value: skill_type.into() });
      let max_level = number(object, "MaxLv"); if !max_level.is_empty() { fields.push(CatalogField { label: "Max level".into(), value: max_level }); }
      let cooldown = skill_metric(object, "cooldown", "seconds"); if !cooldown.is_empty() { fields.push(CatalogField { label: "Cooldown".into(), value: cooldown }); }
      let cost = skill_metric(object, "cost", "mana"); if !cost.is_empty() { fields.push(CatalogField { label: "Cost".into(), value: cost }); }
    } else { let stats = strings(object, &["stats", "statsPrimary", "statsSecondary", "statsFullSet", "skillList"]);
      if !stats.is_empty() { fields.push(CatalogField { label: "Stats".into(), value: stats }); }
      let slots = number(object, "Slots"); if !slots.is_empty() { fields.push(CatalogField { label: "Card slots".into(), value: slots }); }
      let obtained = related(object); if !obtained.is_empty() { fields.push(CatalogField { label: "Obtained from".into(), value: obtained }); }
      let drops = strings(object, &["drops"]); if !drops.is_empty() { fields.push(CatalogField { label: "Drops".into(), value: drops }); }
      if kind == "Cards" || kind == "Gems" {
        let drop_type = if kind == "Cards" { "card" } else { "gem" };
        let sources = [slug.as_str(), aliases.as_str(), name.as_str()].iter().find_map(|key| dropped_by.get(&format!("{drop_type}:{}", key.to_ascii_lowercase())));
        if let Some(sources) = sources { fields.push(CatalogField { label: "Dropped by".into(), value: sources.join("\n") }); }
      }
      if kind == "Monsters" {
        let locations = [slug.as_str(), aliases.as_str(), name.as_str()].iter().find_map(|key| monster_locations.get(&key.to_ascii_lowercase()));
        if let Some(locations) = locations { fields.push(CatalogField { label: "Locations".into(), value: locations.join("\n") }); }
      }
    }
    if fields.is_empty() { fields.push(CatalogField { label: "Catalog".into(), value: format!("Live {kind} record") }); }
    Some(CatalogEntry { id: format!("{path}:{}:{}", if slug.is_empty() { name.to_lowercase().replace(' ', "-") } else { slug }, if aliases.is_empty() { name.to_lowercase().replace(' ', "-") } else { aliases.to_lowercase().replace(' ', "-") }), kind: kind.into(), name, subtitle: if subtitle.is_empty() { kind.into() } else { subtitle }, summary: if summary.is_empty() { format!("Live {kind} record from SpiritVale Info.") } else { summary }, tags: { let mut tags = vec![kind.into(), element]; if kind == "Cards" { tags.push(if is_boss == "1" { "Boss".into() } else { "Normal".into() }); } if kind == "Gems" { tags.push(if is_boss == "1" { "Boss".into() } else { "Skill".into() }); } tags }, source_url: format!("https://www.spiritvale.info/{path}"), aliases, image_url: { let icon = text(object, &["icon"]); if icon.is_empty() { None } else if icon.starts_with("item-") { Some(format!("https://www.spiritvale.info/content/game/icons/{icon}.webp")) } else if kind == "Skills" { Some(format!("https://www.spiritvale.info/content/game/icons/skill-{icon}.webp")) } else { Some(format!("https://www.spiritvale.info/content/game/icons/item-{icon}.webp")) } }, fields })
  }).collect())
}
async fn fetch_database() -> Result<Vec<CatalogEntry>, String> {
  let client = reqwest::Client::builder().user_agent("SpiritVale Overlay/1.0.1 (read-only catalog companion)").timeout(std::time::Duration::from_secs(20)).build().map_err(|error| error.to_string())?;
  let mut pages = Vec::new();
  for (kind, path, property) in SOURCES {
    let body = client.get(format!("https://www.spiritvale.info/{path}")).send().await.map_err(|error| format!("{kind}: {error}"))?.error_for_status().map_err(|error| format!("{kind}: {error}"))?.text().await.map_err(|error| format!("{kind}: {error}"))?;
    pages.push((kind, path, property, body));
  }
  let dropped_by = pages.iter().find(|(kind, _, _, _)| *kind == "Monsters").map(|(_, _, _, body)| monster_drop_index(body)).transpose()?.unwrap_or_default();
  let monster_locations = pages.iter().find(|(kind, _, _, _)| *kind == "Maps").map(|(_, _, _, body)| monster_location_index(body)).transpose()?.unwrap_or_default();
  let mut entries = Vec::new();
  for (kind, path, property, body) in pages { entries.extend(parse_entries(&body, property, kind, path, &dropped_by, &monster_locations)?); }
  Ok(entries)
}
#[tauri::command]
async fn load_catalog(app: AppHandle) -> Result<CatalogResponse, String> { let path = cache_path(&app)?; match fetch_database().await { Ok(entries) => { let synced_at = now_epoch(); fs::write(path, serde_json::to_vec(&CachedCatalog { entries: entries.clone(), synced_at }).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?; Ok(CatalogResponse { entries, source: "live".into(), synced_at }) }, Err(network_error) => fs::read(path).ok().and_then(|bytes| serde_json::from_slice::<CachedCatalog>(&bytes).ok()).map(|cache| CatalogResponse { entries: cache.entries, source: "cached".into(), synced_at: cache.synced_at }).ok_or(network_error) } }
#[tauri::command] fn close_overlay(window: WebviewWindow) { let _ = window.close(); }
#[tauri::command] fn minimize_overlay(window: WebviewWindow) { let _ = window.minimize(); }
#[tauri::command]
fn set_window_size_preset(app: AppHandle, window: WebviewWindow, width: u32, height: u32) -> Result<WindowPreferences, String> {
  if !is_window_size_preset(width, height) { return Err("That window size is not an available preset.".into()); }
  apply_window_size(&window, width, height)?;
  let preferences = WindowPreferences { width, height };
  save_window_preferences(&app, &preferences)?;
  Ok(preferences)
}
#[tauri::command]
fn remember_current_window_size(app: AppHandle, window: WebviewWindow) -> Result<WindowPreferences, String> {
  let physical_size = window.inner_size().map_err(|error| error.to_string())?;
  let scale_factor = window.scale_factor().map_err(|error| error.to_string())?;
  let preferences = WindowPreferences {
    width: ((physical_size.width as f64) / scale_factor).round() as u32,
    height: ((physical_size.height as f64) / scale_factor).round() as u32,
  };
  if preferences.width < 640 || preferences.height < 360 { return Err("The current window is too small to remember.".into()); }
  save_window_preferences(&app, &preferences)?;
  Ok(preferences)
}#[tauri::command] fn begin_drag(window: WebviewWindow) { let _ = window.start_dragging(); }
fn toggle_palette(window: &WebviewWindow) { if window.is_minimized().unwrap_or(false) { let _ = window.unminimize(); let _ = window.show(); let _ = window.set_focus(); } else { let _ = window.minimize(); } }
fn main() { tauri::Builder::default().invoke_handler(tauri::generate_handler![close_overlay, minimize_overlay, begin_drag, set_window_size_preset, remember_current_window_size, load_catalog]).plugin(tauri_plugin_opener::init()).plugin(tauri_plugin_global_shortcut::Builder::new().build()).setup(|app| { if let Some(window) = app.get_webview_window("main") { window.set_icon(tauri::include_image!("icons/icon.png"))?; if let Some(preferences) = saved_window_preferences(&app.handle()) { let _ = apply_window_size(&window, preferences.width, preferences.height); } } let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyE); app.global_shortcut().on_shortcut(shortcut, |app, _, event| { if event.state == ShortcutState::Pressed { if let Some(window) = app.get_webview_window("main") { toggle_palette(&window); } } })?; Ok(()) }).run(tauri::generate_context!()).expect("error while running SpiritVale Overlay"); }
