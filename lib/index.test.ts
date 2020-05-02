import {Decimal, RoundingStrategy} from "./index";

test("Decimal.add", () => {
  const d = Decimal.from("10.5");
  expect(d.add(Decimal.from("20.5"))).toEqual(Decimal.from("31.0"));
});

test("Decimal.round", () => {
  const d = Decimal.from("10.5456");
  expect(d.round(2, RoundingStrategy.HALF_UP)).toEqual(Decimal.from("10.55"));
});

test("Decimal.toString", () => {
  const d = Decimal.from("1.15");
  expect(d.toString()).toBe("1.15");
});
