/**
 * Entity specification for generating complex objects.
 *
 * This class mirrors the Rust Entity struct and provides all the same functionality
 * including count specification, uniqueness constraints, and field definitions.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
} from "../types";
import type { CountSpec } from "./count";
import type { FieldSpec } from "./field";

/**
 * Entity class representing the complete entity specification.
 *
 * Corresponds directly to the Rust Entity struct with all fields:
 * - count: Optional count specification for generating arrays
 * - seed: Optional seed for deterministic generation (planned for future use)
 * - unique_by: Fields that must be unique across generated entities
 * - fields: The field definitions that make up the entity structure
 */
export class Entity {
  /**
   * Optional count specification for the number of entities to generate.
   *
   * Determines whether to generate a single entity object or an array of entities:
   * - undefined: Generates a single entity object (not wrapped in an array)
   * - CountSpec: Generates an array with the specified count
   */
  public count?: CountSpec;

  /**
   * Optional seed for deterministic entity generation.
   *
   * When specified, this seed can be used to ensure reproducible entity generation
   * for testing and debugging purposes. Currently preserved for future implementation
   * of per-entity seeding.
   */
  public seed?: number;

  /**
   * Fields that must be unique across all generated entities.
   *
   * This array specifies which field combinations must be unique when generating
   * multiple entities. The uniqueness check creates a fingerprint from the specified
   * fields and ensures no duplicates are generated.
   *
   * Examples:
   * - [] - No uniqueness constraints (default)
   * - ["email"] - Ensures email field is unique across entities
   * - ["user_id", "project_id"] - Ensures the combination is unique
   */
  public unique_by: string[] = [];

  /**
   * The collection of fields that make up the entity structure.
   *
   * This Record defines the schema for the generated entities, mapping field
   * names to their generation specifications. Field ordering is preserved
   * based on object key insertion order.
   */
  public fields: Record<string, FieldSpec>;

  /**
   * Creates a new Entity instance.
   *
   * @param fields - The field definitions for this entity
   * @param count - Optional count specification for generating arrays
   * @param seed - Optional seed for deterministic generation
   * @param unique_by - Fields that must be unique across entities
   */
  constructor(
    fields: Record<string, FieldSpec>,
    count?: CountSpec,
    seed?: number,
    unique_by: string[] = []
  ) {
    this.fields = fields;
    this.count = count;
    this.seed = seed;
    this.unique_by = unique_by;
  }

  /**
   * Creates an Entity instance from a plain object specification.
   *
   * @param spec - Plain object containing entity specification
   * @returns New Entity instance
   */
  static fromSpec(spec: {
    count?: CountSpec;
    seed?: number;
    unique_by?: string[];
    fields: Record<string, FieldSpec>;
  }): Entity {
    return new Entity(spec.fields, spec.count, spec.seed, spec.unique_by || []);
  }

  /**
   * Converts this Entity to a plain object specification.
   *
   * @returns Plain object representation
   */
  toSpec(): {
    count?: CountSpec;
    seed?: number;
    unique_by?: string[];
    fields: Record<string, FieldSpec>;
  } {
    const spec: any = {
      fields: this.fields,
    };

    if (this.count !== undefined) {
      spec.count = this.count;
    }

    if (this.seed !== undefined) {
      spec.seed = this.seed;
    }

    if (this.unique_by.length > 0) {
      spec.unique_by = this.unique_by;
    }

    return spec;
  }

  /**
   * Returns a string representation of this Entity.
   */
  toString(): string {
    const fieldCount = Object.keys(this.fields).length;
    const hasCount = this.count !== undefined;
    const hasUniqueBy = this.unique_by.length > 0;
    return `Entity(fields: ${fieldCount}, hasCount: ${hasCount}, uniqueBy: ${hasUniqueBy})`;
  }
}

/**
 * Legacy type alias for backward compatibility.
 * @deprecated Use Entity class instead
 */
export type EntitySpec = {
  count?: CountSpec;
  seed?: number;
  unique_by?: string[];
  fields: Record<string, FieldSpec>;
};
