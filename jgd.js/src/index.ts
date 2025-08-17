/**
 * JGD.js - JSON Generator Definition TypeScript Library
 *
 * A TypeScript library for generating realistic JSON data using declarative schema definitions.
 * Based on the Rust JGD library architecture with TypeScript best practices.
 */

// Main exports
export { Jgd, generateFromString, generateFromFile } from "./jgd";

// Type exports
export type {
  JsonValue,
  GenerationResult,
  Arguments,
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
export { NumberSpec } from "./number-spec";
export { ArraySpec } from "./array-spec";
export { OptionalSpec } from "./optional-spec";
export { FieldGenerator } from "./field-generator";
export { EntityGenerator } from "./entity-generator";
export { Replacer, ArgumentsHelper } from "./replacer";

// Template processing exports
export { processTemplate, isTemplate } from "./template";

// JGD Keys exports
export { JgdKeyGenerator, JGD_KEYS } from "./keys";

// Error export
export { JgdGeneratorError } from "./types";

// Version and metadata
export const VERSION = "0.2.0";
export const FORMAT = "jgd/v1";

/**
 * Default export for convenience
 */
import { Jgd } from "./jgd";
export default Jgd;
