/**
 * Supported locale codes.
 */

export type LocaleCode =
  | "EN"
  | "FR_FR"
  | "IT_IT"
  | "JA_JP"
  | "DE_DE"
  | "PT_BR"
  | "AR_SA"
  | "CY_GB";

const SUPPORTED_LOCALES: LocaleCode[] = [
  "EN",
  "FR_FR",
  "IT_IT",
  "JA_JP",
  "DE_DE",
  "PT_BR",
  "AR_SA",
  "CY_GB",
];

/**
 * Convert string to valid locale code.
 */
export function localeFromString(value: string): LocaleCode {
  // Check if the value is a valid locale code
  if (SUPPORTED_LOCALES.includes(value as LocaleCode)) {
    return value as LocaleCode;
  }
  // Default to EN if not found
  return "EN";
}

/**
 * Get all supported locale codes.
 */
export function getSupportedLocales(): readonly LocaleCode[] {
  return SUPPORTED_LOCALES;
}

/**
 * Check if a locale code is supported.
 */
export function isLocaleSupported(locale: string): boolean {
  return SUPPORTED_LOCALES.includes(locale as LocaleCode);
}
