const assert = require("assert");
const bindings = require("bindings");
const addon = bindings("addon");

const result = addon.register();
assert.strictEqual(result.foo, "bar");
