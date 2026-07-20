export type EntryKind = "Monster" | "Boss" | "Card" | "Item" | "Map";

export type CatalogEntry = {
  id: string;
  kind: EntryKind;
  name: string;
  subtitle: string;
  summary: string;
  tags: string[];
  sourceUrl: string;
  fields: Array<{ label: string; value: string }>;
};

// Deliberately fictional records used until the official-data adapter is built.
export const demoCatalog: CatalogEntry[] = [
  { id: "mossling", kind: "Monster", name: "Mossling", subtitle: "Level 12 · Earth", summary: "A quiet forest creature whose drops are useful for early crafting.", tags: ["forest", "earth", "crafting"], sourceUrl: "https://spiritvale.info/monsters", fields: [{ label: "Found in", value: "Verdant Hollow" }, { label: "Key drops", value: "Moss Fibre · Verdant Card" }, { label: "Weakness", value: "Fire" }] },
  { id: "cinder-warden", kind: "Boss", name: "Cinder Warden", subtitle: "Level 58 · Fire", summary: "A fictional volcanic sentinel used to preview boss result cards.", tags: ["boss", "fire", "volcano"], sourceUrl: "https://spiritvale.info/monsters", fields: [{ label: "Found in", value: "Ashfall Crucible" }, { label: "Key drops", value: "Cinder Sigil · Emberplate" }, { label: "Weakness", value: "Water" }] },
  { id: "verdant-card", kind: "Card", name: "Verdant Card", subtitle: "Chest · Utility", summary: "A sample card that demonstrates the searchable official catalog layout.", tags: ["card", "chest", "forest"], sourceUrl: "https://spiritvale.info/cards", fields: [{ label: "Effect", value: "Max HP +4%" }, { label: "Source", value: "Mossling" }, { label: "Affix", value: "Verdant" }] },
  { id: "emberplate", kind: "Item", name: "Emberplate", subtitle: "Chest armour · Rare", summary: "A fictional equipment entry for validating item search and pinning.", tags: ["item", "armour", "fire"], sourceUrl: "https://spiritvale.info", fields: [{ label: "Source", value: "Cinder Warden" }, { label: "Bonus", value: "Fire resistance +12%" }, { label: "Type", value: "Chest armour" }] },
  { id: "verdant-hollow", kind: "Map", name: "Verdant Hollow", subtitle: "Forest biome", summary: "A fictional map used to test map-to-monster relationships.", tags: ["map", "forest", "mossling"], sourceUrl: "https://spiritvale.info/maps", fields: [{ label: "Creatures", value: "Mossling · Briarling" }, { label: "Biome", value: "Forest" }, { label: "Notable drop", value: "Verdant Card" }] }
];
