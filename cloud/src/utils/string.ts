export function substringAfter(str: string, substr: string): string {
  const index = str.indexOf(substr);
  if (index === -1) {
    return "";
  }
  return str.slice(index + substr.length);
}
