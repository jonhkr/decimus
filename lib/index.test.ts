
import decimal from "./index";

test("str", () => {
  const d = decimal.new("10");
  expect(decimal.str(d)).toBe("10");
});
