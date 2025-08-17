/**
 * JGD.js - JSON Generator Definition TypeScript Library
 *
 * A TypeScript library for generating realistic JSON data using declarative schema definitions.
 * Based on the Rust JGD library architecture with TypeScript best practices.
 */

// Main exports
export { Jgd, generateFromString, generateFromFile } from "./type-spec/jgd";

// Type exports
export type {
  JsonValue,
  GenerationResult,
  Arguments as ArgumentsType,
  CustomKeyFunction,
  CountSpec,
  NumberSpecInput,
  ArraySpecInput,
  OptionalSpecInput,
  FieldSpec,
  EntitySpec,
  JgdSchema,
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
} from "./utils/mod";

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
export { EntityGenerator } from "./type-spec/entity";
export { Replacer, Arguments } from "./utils/mod";

// Template processing exports
export { processTemplate, isTemplate } from "./template";

// JGD Keys exports
export { JgdKeyGenerator, JGD_KEYS } from "./fake/mod";

// Error export
export { JgdGeneratorError } from "./types";

// Locales export
export {
  LocalesKeys,
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
