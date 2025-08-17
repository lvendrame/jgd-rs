/**
 * JGD.js - JSON Generator Definition TypeScript Library
 *
 * A TypeScript library for generating realistic JSON data using declarative schema definitions.
 * Based on the Rust JGD library architecture with TypeScript best practices.
 */

// Main exports - including new structured classes
export { Jgd, generateFromString, generateFromFile } from "./type-spec/jgd";

// New structured type exports that mirror Rust exactly
export { Entity } from "./type-spec/entity";
export { Jgd as JgdClass } from "./type-spec/jgd-schema";
export {
  Count,
  resolveCount as resolveCountSpec,
  type CountSpec,
} from "./type-spec/count";

// Type exports from type-spec modules
export type { NumberSpecInput } from "./type-spec/number-spec";
export type { ArraySpecInput } from "./type-spec/array-spec";
export type { OptionalSpecInput } from "./type-spec/optional-spec";
export type { FieldSpec } from "./type-spec/field";

// Type exports - maintaining backward compatibility with core types only
export type {
  JsonValue,
  GenerationResult,
  Arguments as ArgumentsType,
  CustomKeyFunction,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
} from "./types";

// Utility exports
export {
  success,
  error,
  resolveCount,
  parseArguments,
  getStringFromArgs,
  getNumberFromArgs,
  getNumberRangeFromArgs,
  validateSchemaMode,
  validateNumberRange,
  toJsonValue,
} from "./utils";

// Type guard exports
export { isSuccess, isError } from "./types";

// Configuration exports
export {
  createGeneratorConfig,
  createConfigWithGlobalKeys,
  addCustomKey,
  getCustomKey,
  globalCustomKeyRegistry,
  type SupportedLocale,
} from "./config";

// Generator class exports
export { NumberSpec } from "./type-spec/number-spec";
export { ArraySpec } from "./type-spec/array-spec";
export { OptionalSpec } from "./type-spec/optional-spec";
export { FieldGenerator } from "./type-spec/field";

// Utility class exports
export { Replacer, Arguments } from "./utils";

// Template processing exports
export { processTemplate, isTemplate } from "./template";

// JGD Keys exports
export { JgdKeyGenerator, JGD_KEYS } from "./fake";

// Error export
export { JgdGeneratorError } from "./types";

// Locales export
export type { LocaleCode } from "./locales-keys";
export {
  localeFromString,
  getSupportedLocales,
  isLocaleSupported,
} from "./locales-keys";

// Version and metadata
export const VERSION = "0.2.0";
export const FORMAT = "jgd/v1";

/**
 * Default export for convenience
 */
import { Jgd } from "./type-spec/jgd";
export default Jgd;
