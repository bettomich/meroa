import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from "react";
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
const LANGUAGE_STORAGE_KEY = "meroa.language.v1";

function initialLanguage(): Language {
  try {
    const stored = window.localStorage.getItem(LANGUAGE_STORAGE_KEY);
    if (stored === "en" || stored === "it") return stored;
  } catch {
    // Fall back to the operating-system language when storage is unavailable.
  }
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
  const [language, setLanguageState] = useState<Language>(initialLanguage);
  const setLanguage = useCallback((nextLanguage: Language) => {
    setLanguageState(nextLanguage);
    try {
      window.localStorage.setItem(LANGUAGE_STORAGE_KEY, nextLanguage);
    } catch {
      // The selected language remains active for this application session.
    }
  }, []);
  const value = useMemo<I18nValue>(() => ({
    language,
    setLanguage,
    t: (key) => resolve(resources[language], key) ?? resolve(resources.en, key) ?? key,
  }), [language, setLanguage]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nValue {
  const value = useContext(I18nContext);
  if (!value) throw new Error("useI18n must be used within I18nProvider");
  return value;
}
