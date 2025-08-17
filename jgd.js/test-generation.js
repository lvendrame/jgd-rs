const { Jgd } = require("./dist");

const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  root: {
    count: 3,
    fields: {
      id: "${ulid}",
      name: "${name.firstName}",
      email: "${internet.safeEmail}",
    },
  },
};

console.log("Testing JGD generation...");
const jgd = Jgd.fromObject(schema);
console.log("JGD created successfully");

const result = jgd.generate();
console.log("Generation result:", result);
if (!result.success) {
  console.log("Error:", result.error);
} else {
  console.log("Generated data:", JSON.stringify(result.data, null, 2));
}
