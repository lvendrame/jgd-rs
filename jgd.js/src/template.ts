/**
 * Template processor for handling strings with embedded faker patterns.
 */

import type {
  GeneratorConfig,
  LocalConfig,
  GenerationResult,
  JsonValue,
} from "./types";
import { success, error } from "./utils/generator-utils";
import { Replacer } from "./utils/replacer";

/**
 * Regular expression for finding all ${...} patterns in a string.
 */
const TEMPLATE_PATTERN_REGEX = /\$\{([^}]+)\}/g;

/**
 * Processes a template string by replacing all ${...} patterns with generated values.
 */
export function processTemplate(
  template: string,
  config: GeneratorConfig,
  localConfig?: LocalConfig
): GenerationResult<JsonValue> {
  try {
    // Find all ${...} patterns
    const patterns = Array.from(template.matchAll(TEMPLATE_PATTERN_REGEX));

    if (patterns.length === 0) {
      // No patterns found, return as-is
      return success(template);
    }

    // Special case: if the entire template is a single pattern, return the raw value
    if (patterns.length === 1 && patterns[0][0] === template) {
      const fullMatch = patterns[0][0];
      const replacer = Replacer.from(fullMatch);
      return replacer.generateValue(config, localConfig);
    }

    let result = template;

    // Process each pattern for string templates
    for (const match of patterns) {
      const fullMatch = match[0]; // e.g., "${index}"

      const replacer = Replacer.from(fullMatch);
      const valueResult = replacer.generateValue(config, localConfig);

      if (!valueResult.success) {
        return error(
          `Template processing failed for pattern '${fullMatch}': ${
            (valueResult as any).error
          }`
        );
      }

      // Replace the pattern with the generated value
      result = result.replace(fullMatch, String(valueResult.data));
    }

    return success(result);
  } catch (err) {
    return error(
      `Template processing failed: ${
        err instanceof Error ? err.message : String(err)
      }`
    );
  }
}

/**
 * Checks if a string contains template patterns.
 */
export function isTemplate(str: string): boolean {
  const regex = /\$\{([^}]+)\}/;
  return regex.test(str);
}
