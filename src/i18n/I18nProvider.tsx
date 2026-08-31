import { createContext, useContext, useMemo, useState, type ReactNode } from "react";
import en from "../locales/en.json";
import it from "../locales/it.json";

type Language = "en" | "it";
interface TranslationTree {
  [key: string]: string | TranslationTree;
}

interface I18nValue {
  language: Language;
  setLanguage: (language: Language) => void;
  t: (key: string) => string;
}

const resources: Record<Language, TranslationTree> = { en, it };
const I18nContext = createContext<I18nValue | null>(null);

function initialLanguage(): Language {
  const preferred = navigator.languages?.[0] ?? navigator.language;
  return preferred.toLowerCase().startsWith("it") ? "it" : "en";
}

function resolve(tree: TranslationTree, key: string): string | undefined {
  let current: string | TranslationTree = tree;
  for (const part of key.split(".")) {
    if (typeof current === "string" || !(part in current)) return undefined;
    current = current[part];
  }
  return typeof current === "string" ? current : undefined;
}

export function I18nProvider({ children }: { children: ReactNode }) {
  const [language, setLanguage] = useState<Language>(initialLanguage);
  const value = useMemo<I18nValue>(() => ({
    language,
    setLanguage,
    t: (key) => resolve(resources[language], key) ?? resolve(resources.en, key) ?? key,
  }), [language]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nValue {
  const value = useContext(I18nContext);
  if (!value) throw new Error("useI18n must be used within I18nProvider");
  return value;
}
