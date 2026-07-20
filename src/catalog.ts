export type CatalogEntry = {
  id: string;
  kind: string;
  name: string;
  subtitle: string;
  summary: string;
  tags: string[];
  sourceUrl: string;
  imageUrl?: string;
  aliases?: string;
  fields: Array<{ label: string; value: string }>;
};

export type CatalogResponse = {
  entries: CatalogEntry[];
  source: "live" | "cached";
  syncedAt: number;
};
