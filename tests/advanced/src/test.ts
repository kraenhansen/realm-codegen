import { expect } from "chai";

import { Constructable, Stringer, Objectifier } from ".";

describe("Constructable", () => {
  it("constructs", () => {
    const constructable = new Constructable(123);
    const result = constructable.getNumber();
    expect(result).equals(123);
  });
});

describe("Stringer", () => {
  it("uppercases", () => {
    const stringer = new Stringer();
    const result = stringer.uppercase("hello!");
    expect(result).equals("HELLO!");
  });
});

describe("Objectifier", () => {
  it("builds objects", () => {
    const objectifier = new Objectifier({
      aGreeting: "Hello",
      aNumber: 123,
    });
    const result = objectifier.build({
      aGreeting: "World!",
      aNumber: 321
    });
    expect(result).deep.equals({
      aGreeting: "HelloWorld!",
      aNumber: 123+321,
    });
  });
});
