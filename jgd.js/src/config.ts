/**
 * Generator configuration factory and management.
 */

import { faker, Faker } from "@faker-js/faker";
import type { GeneratorConfig, CustomKeyFunction } from "./types";

/**
 * Available locale codes supported by JGD.
 */
export type SupportedLocale =
  | "EN"
  | "FR_FR"
  | "IT_IT"
  | "JA_JP"
  | "DE_DE"
  | "PT_BR"
  | "AR_SA"
  | "CY_GB";

/**
 * Creates a new GeneratorConfig instance with the specified locale and seed.
 */
export function createGeneratorConfig(
  locale: string = "EN",
  seed?: number,
  customKeys?: Map<string, CustomKeyFunction>
): GeneratorConfig {
  // Create faker instance
  const fakerInstance = createFakerInstance(locale, seed);

  return {
    locale: locale.toUpperCase(),
    seed,
    faker: fakerInstance,
    customKeys: customKeys || new Map(),
    depth: 0,
    indexStack: [],
    countStack: [],
    entityNameStack: [],
    fieldNameStack: [],
  };
}

/**
 * Creates a faker instance with the specified locale and seed.
 */
function createFakerInstance(locale: string, seed?: number): Faker {
  // Normalize locale to lowercase for faker
  const normalizedLocale = locale.toLowerCase();

  // Create faker instance - for now we'll use the default faker
  // and document that locale support is limited in this version
  let fakerInstance: Faker = faker;

  try {
    // Note: In this implementation, we use the default English faker
    // Full locale support would require importing specific faker locales
    if (normalizedLocale !== "en" && normalizedLocale !== "english") {
      console.warn(
        `Locale '${locale}' requested but using English faker. Full locale support requires additional configuration.`
      );
    }
  } catch (err) {
    console.warn(`Failed to process locale '${locale}', using English:`, err);
  }

  // Set seed if provided
  if (seed !== undefined) {
    fakerInstance.seed(seed);
  }

  return fakerInstance;
}

/**
 * Global registry for custom key functions.
 */
class CustomKeyRegistry {
  private keys = new Map<string, CustomKeyFunction>();

  /**
   * Adds a custom key function to the global registry.
   */
  addCustomKey(key: string, func: CustomKeyFunction): void {
    this.keys.set(key, func);
  }

  /**
   * Gets a custom key function from the global registry.
   */
  getCustomKey(key: string): CustomKeyFunction | undefined {
    return this.keys.get(key);
  }

  /**
   * Gets all custom keys as a new Map.
   */
  getAllCustomKeys(): Map<string, CustomKeyFunction> {
    return new Map(this.keys);
  }

  /**
   * Removes a custom key from the registry.
   */
  removeCustomKey(key: string): boolean {
    return this.keys.delete(key);
  }

  /**
   * Clears all custom keys from the registry.
   */
  clearCustomKeys(): void {
    this.keys.clear();
  }
}

/**
 * Global instance of the custom key registry.
 */
export const globalCustomKeyRegistry = new CustomKeyRegistry();

/**
 * Adds a custom key function to the global registry.
 * This function will be available in all JGD instances.
 */
export function addCustomKey(key: string, func: CustomKeyFunction): void {
  globalCustomKeyRegistry.addCustomKey(key, func);
}

/**
 * Gets a custom key function from the global registry.
 */
export function getCustomKey(key: string): CustomKeyFunction | undefined {
  return globalCustomKeyRegistry.getCustomKey(key);
}

/**
 * Creates a GeneratorConfig with all global custom keys included.
 */
export function createConfigWithGlobalKeys(
  locale: string = "EN",
  seed?: number
): GeneratorConfig {
  const customKeys = globalCustomKeyRegistry.getAllCustomKeys();
  return createGeneratorConfig(locale, seed, customKeys);
}
