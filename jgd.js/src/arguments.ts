/**
 * Arguments parser and processor - matches Rust Arguments exactly.
 *
 * Represents parsed arguments from faker pattern parameters.
 * Arguments are typically extracted from parentheses in faker patterns like:
 * - `faker.name.first_name()` → `Arguments.None`
 * - `faker.number.number(100)` → `Arguments.Fixed("100")`
 * - `faker.number.between(1,10)` → `Arguments.Range("1", "10")`
 * - `faker.date.between(2020-01-01..2024-12-31)` → `Arguments.Range("2020-01-01", "2024-12-31")`
 */

/**
 * Arguments enum representing different types of parsed arguments.
 */
export enum ArgumentsType {
  None = "none",
  Fixed = "fixed",
  Range = "range",
}

/**
 * Arguments class with helper methods for extracting typed values.
 */
export class Arguments {
  public readonly type: ArgumentsType;
  public readonly value?: string;
  public readonly min?: string;
  public readonly max?: string;

  constructor(type: ArgumentsType, value?: string, min?: string, max?: string) {
    this.type = type;
    this.value = value;
    this.min = min;
    this.max = max;
  }

  /**
   * Creates Arguments from a string representation.
   *
   * Parses a string into Arguments enum.
   * Expects input in the format `(content)` where content can be:
   * - Empty: `()` → `Arguments.None`
   * - Single value: `(42)` → `Arguments.Fixed("42")`
   * - Comma-separated: `(1,10)` → `Arguments.Range("1", "10")`
   * - Dot-separated: `(1..10)` → `Arguments.Range("1", "10")`
   */
  static from(value: string): Arguments {
    const trimmed = value.trim();

    // Extract content from parentheses
    const match = trimmed.match(/^\((.*)?\)$/);
    if (!match) {
      return new Arguments(ArgumentsType.None);
    }

    const argsContent = match[1] || "";

    if (!argsContent.trim()) {
      return new Arguments(ArgumentsType.None);
    }

    // Split by .. or , to handle ranges
    let parts: string[];
    if (argsContent.includes("..")) {
      parts = argsContent.split("..").map((s) => s.trim());
    } else if (argsContent.includes(",")) {
      parts = argsContent.split(",").map((s) => s.trim());
    } else {
      parts = [argsContent.trim()];
    }

    if (parts.length > 1) {
      if (parts[0] === "" && parts[1] === "") {
        return new Arguments(ArgumentsType.None);
      }

      if (parts[1] === "") {
        return new Arguments(ArgumentsType.Fixed, parts[0]);
      }

      return new Arguments(ArgumentsType.Range, undefined, parts[0], parts[1]);
    }

    if (parts[0] !== "") {
      return new Arguments(ArgumentsType.Fixed, parts[0]);
    }

    return new Arguments(ArgumentsType.None);
  }

  /**
   * Helper function to parse a single numeric argument.
   */
  private static parseNumber<T extends number>(
    arg: string,
    defaultValue: T
  ): T {
    const parsed = parseFloat(arg);
    return isNaN(parsed) ? defaultValue : (parsed as T);
  }

  /**
   * Helper function to parse a datetime argument.
   */
  private static parseDateTime(arg: string, defaultValue: Date): Date {
    // Try parsing as ISO 8601 first
    let date = new Date(arg);
    if (!isNaN(date.getTime())) {
      return date;
    }

    // Try common formats
    const formats = [
      // ISO formats
      /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{3})?Z?$/,
      // Space separated
      /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}(?:\.\d{3})?$/,
      // Date only
      /^\d{4}-\d{2}-\d{2}$/,
    ];

    for (const format of formats) {
      if (format.test(arg)) {
        date = new Date(arg);
        if (!isNaN(date.getTime())) {
          return date;
        }
      }
    }

    return defaultValue;
  }

  /**
   * Extracts a string value from the arguments.
   *
   * Returns the first argument for Fixed and Range variants,
   * or the default value for None variant.
   */
  getString(defaultValue: string): string {
    switch (this.type) {
      case ArgumentsType.None:
        return defaultValue;
      case ArgumentsType.Fixed:
        return this.value || defaultValue;
      case ArgumentsType.Range:
        return this.min || defaultValue;
    }
  }

  /**
   * Extracts a tuple of string values from the arguments.
   *
   * Returns both start and end values for different argument types.
   */
  getStringTuple(defaultStart: string, defaultEnd: string): [string, string] {
    switch (this.type) {
      case ArgumentsType.None:
        return [defaultStart, defaultEnd];
      case ArgumentsType.Fixed:
        return [this.value || defaultStart, defaultEnd];
      case ArgumentsType.Range:
        return [this.min || defaultStart, this.max || defaultEnd];
    }
  }

  /**
   * Extracts a numeric value from the arguments.
   *
   * Attempts to parse the first argument as a number. If parsing fails
   * or no arguments are present, returns the default value.
   */
  getNumber<T extends number>(defaultValue: T): T {
    switch (this.type) {
      case ArgumentsType.None:
        return defaultValue;
      case ArgumentsType.Fixed:
        return Arguments.parseNumber(this.value || "", defaultValue);
      case ArgumentsType.Range:
        return Arguments.parseNumber(this.min || "", defaultValue);
    }
  }

  /**
   * Extracts a numeric range from the arguments.
   *
   * Creates a tuple from the arguments, parsing both start and end values.
   * If parsing fails, uses the corresponding default value.
   */
  getNumberRange<T extends number>(defaultStart: T, defaultEnd: T): [T, T] {
    switch (this.type) {
      case ArgumentsType.None:
        return [defaultStart, defaultEnd];
      case ArgumentsType.Fixed:
        return [
          Arguments.parseNumber(this.value || "", defaultStart),
          defaultEnd,
        ];
      case ArgumentsType.Range:
        return [
          Arguments.parseNumber(this.min || "", defaultStart),
          Arguments.parseNumber(this.max || "", defaultEnd),
        ];
    }
  }

  /**
   * Extracts a datetime value from the arguments.
   *
   * Attempts to parse the first argument as a Date using multiple formats.
   * If parsing fails or no arguments are present, returns the default value.
   */
  getDateTime(defaultValue: Date): Date {
    switch (this.type) {
      case ArgumentsType.None:
        return defaultValue;
      case ArgumentsType.Fixed:
        return Arguments.parseDateTime(this.value || "", defaultValue);
      case ArgumentsType.Range:
        return Arguments.parseDateTime(this.min || "", defaultValue);
    }
  }

  /**
   * Extracts a datetime range from the arguments.
   *
   * Creates a tuple of Date values from the arguments, parsing both start and end values.
   * If parsing fails, uses the corresponding default value.
   */
  getDateTimeRange(defaultStart: Date, defaultEnd: Date): [Date, Date] {
    switch (this.type) {
      case ArgumentsType.None:
        return [defaultStart, defaultEnd];
      case ArgumentsType.Fixed:
        return [
          Arguments.parseDateTime(this.value || "", defaultStart),
          defaultEnd,
        ];
      case ArgumentsType.Range:
        return [
          Arguments.parseDateTime(this.min || "", defaultStart),
          Arguments.parseDateTime(this.max || "", defaultEnd),
        ];
    }
  }

  /**
   * Returns a string representation of the arguments.
   */
  toString(): string {
    switch (this.type) {
      case ArgumentsType.None:
        return "()";
      case ArgumentsType.Fixed:
        return `(${this.value})`;
      case ArgumentsType.Range:
        return `(${this.min}, ${this.max})`;
    }
  }
}
