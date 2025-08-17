/**
 * Locale keys enum - matches the Rust LocalesKeys exactly.
 */

export enum LocalesKeys {
  En = "EN",
  FrFr = "FR_FR",
  ItIt = "IT_IT",
  JaJp = "JA_JP",
  DeDe = "DE_DE",
  PtBr = "PT_BR",
  ArSa = "AR_SA",
  CyGb = "CY_GB",
}

/**
 * Convert string to LocalesKeys enum.
 */
export function localeFromString(value: string): LocalesKeys {
  switch (value) {
    case "EN":
      return LocalesKeys.En;
    case "FR_FR":
      return LocalesKeys.FrFr;
    case "IT_IT":
      return LocalesKeys.ItIt;
    case "JA_JP":
      return LocalesKeys.JaJp;
    case "DE_DE":
      return LocalesKeys.DeDe;
    case "PT_BR":
      return LocalesKeys.PtBr;
    case "AR_SA":
      return LocalesKeys.ArSa;
    case "CY_GB":
      return LocalesKeys.CyGb;
    default:
      return LocalesKeys.En;
  }
}

/**
 * Get all supported locale codes.
 */
export function getSupportedLocales(): string[] {
  return Object.values(LocalesKeys);
}

/**
 * Check if a locale code is supported.
 */
export function isLocaleSupported(locale: string): boolean {
  return Object.values(LocalesKeys).includes(locale as LocalesKeys);
}
