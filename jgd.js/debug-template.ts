#!/usr/bin/env node

import { isTemplate, processTemplate } from "./src/template.js";
import { createGeneratorConfig } from "./src/config.js";

const config = createGeneratorConfig("EN", 42);

console.log("Testing isTemplate:");
console.log('isTemplate("${index}"):', isTemplate("${index}"));
console.log('isTemplate("User ${index}"):', isTemplate("User ${index}"));
console.log('isTemplate("plain text"):', isTemplate("plain text"));

console.log("\nTesting processTemplate:");
const test1 = processTemplate("${index}", config);
console.log('processTemplate("${index}"):', test1);

const test2 = processTemplate("User ${index} of ${count}", config);
console.log('processTemplate("User ${index} of ${count}"):', test2);
