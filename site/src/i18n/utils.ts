import { Locale, defaultLocale } from './config';
import zhTranslations from './locales/zh.json';
import enTranslations from './locales/en.json';

const translations = {
  zh: zhTranslations,
  en: enTranslations,
};

export function getTranslations(locale: Locale) {
  return translations[locale] || translations[defaultLocale];
}

export function getNestedTranslation(obj: any, path: string): any {
  return path.split('.').reduce((current, key) => current?.[key], obj) || path;
}

export function useTranslations(locale: Locale) {
  const t = getTranslations(locale);
  
  return (key: string): any => {
    return getNestedTranslation(t, key);
  };
}
