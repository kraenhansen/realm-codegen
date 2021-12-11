import { expect } from "chai";

import { Constructable, Stringer } from ".";

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
