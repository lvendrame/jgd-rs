#!/usr/bin/env node

import { processTemplate } from "./src/template.js";
import { createGeneratorConfig } from "./src/config.js";

const config = createGeneratorConfig("EN", 42);

console.log("Testing template processing:");

const result1 = processTemplate("User ${index} of ${count}", config);
console.log("Result 1:", result1);

const result2 = processTemplate("${person.firstName}", config);
console.log("Result 2:", result2);

const result3 = processTemplate("Hello ${custom.status}", config);
console.log("Result 3:", result3);
