import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import en from "locales/en.json";
import zh from "locales/zh.json";

const resources = {
  en,
  zh,
};

i18n.use(initReactI18next).init({
  resources,
  lng: "zh",
  interpolation: {
    escapeValue: false,
  },
});

const format = (key: string, substitutions?: { [key: string]: string }) => {
  return i18n.t(key, substitutions);
};

export default format;
