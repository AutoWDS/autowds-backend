import i18n from "i18n";

export function optionsForEnumDesc<T extends Record<string, string>>(type: T) {
  return Object.entries(type).map(([key, desc]) => ({
    label: i18n(desc),
    value: key,
  }));
}
