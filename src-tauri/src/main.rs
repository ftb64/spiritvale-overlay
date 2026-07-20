#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{fs, time::{SystemTime, UNIX_EPOCH}};
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
fn related(object: &Map<String, Value>) -> String { object.get("drops").and_then(Value::as_array).map(|values| values.iter().filter_map(|value| { let item = value.as_object()?; let nested = item.get("monster").and_then(Value::as_object).unwrap_or(item); let name = text(nested, &["name", "DisplayName", "Name"]); if name.is_empty() { return None; } let chance = item.get("chance").and_then(Value::as_f64); Some(chance.map(|value| format!("{name} {value}%")).unwrap_or(name)) }).collect::<Vec<_>>().join("\n")).unwrap_or_default() }
fn parse_entries(payload: &str, property: &str, kind: &str, path: &str) -> Result<Vec<CatalogEntry>, String> {
  let capture = Regex::new(r#"data-page=\"([^\"]+)\""#).expect("valid page regex").captures(payload).ok_or("SpiritVale Info did not provide database data.")?;
  let page: Value = serde_json::from_str(&capture[1].replace("&quot;", "\"").replace("&amp;", "&")).map_err(|error| error.to_string())?;
  let records = page.pointer(&format!("/props/{property}")).and_then(Value::as_array).ok_or_else(|| format!("SpiritVale Info did not provide {kind} records."))?;
  Ok(records.iter().filter_map(|record| {
    let object = record.as_object()?;
    let name = text(object, &["DisplayName", "name", "Name", "GameId"]); if name.is_empty() { return None; }
    let slug = text(object, &["Slug", "slug"]); let aliases = text(object, &["GameId", "Id"]); let level = number(object, "Level"); let element = text(object, &["Element", "typeName", "slot", "Type"]);
    let is_boss = number(object, "IsBoss"); let subtitle = [if level.is_empty() { String::new() } else { format!("Level {level}") }, element.clone()].into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" · ");
    let summary = text(object, &["Description", "description"]);
    let stats = strings(object, &["stats", "statsPrimary", "statsSecondary", "statsFullSet", "skillList"]);
    let mut fields = Vec::new(); if !stats.is_empty() { fields.push(CatalogField { label: "Stats".into(), value: stats }); }
    let slots = number(object, "Slots"); if !slots.is_empty() { fields.push(CatalogField { label: "Card slots".into(), value: slots }); }
    let obtained = related(object); if !obtained.is_empty() { fields.push(CatalogField { label: "Obtained from".into(), value: obtained }); }
    let drops = strings(object, &["drops"]); if !drops.is_empty() { fields.push(CatalogField { label: "Drops".into(), value: drops }); }
    if fields.is_empty() { fields.push(CatalogField { label: "Catalog".into(), value: format!("Live {kind} record") }); }
    Some(CatalogEntry { id: format!("{path}:{}:{}", if slug.is_empty() { name.to_lowercase().replace(' ', "-") } else { slug }, if aliases.is_empty() { name.to_lowercase().replace(' ', "-") } else { aliases.to_lowercase().replace(' ', "-") }), kind: kind.into(), name, subtitle: if subtitle.is_empty() { kind.into() } else { subtitle }, summary: if summary.is_empty() { format!("Live {kind} record from SpiritVale Info.") } else { summary }, tags: { let mut tags = vec![kind.into(), element]; if kind == "Cards" { tags.push(if is_boss == "1" { "Boss".into() } else { "Normal".into() }); } if kind == "Gems" { tags.push(if is_boss == "1" { "Boss".into() } else { "Skill".into() }); } tags }, source_url: format!("https://www.spiritvale.info/{path}"), aliases, image_url: { let icon = text(object, &["icon"]); if icon.is_empty() { None } else if icon.starts_with("item-") { Some(format!("https://www.spiritvale.info/content/game/icons/{icon}.webp")) } else if kind == "Skills" { Some(format!("https://www.spiritvale.info/content/game/icons/skill-{icon}.webp")) } else { Some(format!("https://www.spiritvale.info/content/game/icons/item-{icon}.webp")) } }, fields })
  }).collect())
}
async fn fetch_database() -> Result<Vec<CatalogEntry>, String> {
  let client = reqwest::Client::builder().user_agent("SpiritVale Overlay/0.1.8 (read-only catalog companion)").timeout(std::time::Duration::from_secs(20)).build().map_err(|error| error.to_string())?;
  let mut entries = Vec::new();
  for (kind, path, property) in SOURCES { let body = client.get(format!("https://www.spiritvale.info/{path}")).send().await.map_err(|error| format!("{kind}: {error}"))?.error_for_status().map_err(|error| format!("{kind}: {error}"))?.text().await.map_err(|error| format!("{kind}: {error}"))?; entries.extend(parse_entries(&body, property, kind, path)?); }
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
