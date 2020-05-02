const native = require("../native/index.node"); // eslint-disable-line

type NativeRepr = [number, ...number[]] & { length: 16 };

export enum RoundingStrategy {
  HALF_UP
}

export class Decimal {
  private constructor(private readonly repr: NativeRepr) {}

  static from(input: string): Decimal {
    return new Decimal(native.new(input));
  }

  add(other: Decimal): Decimal {
    return new Decimal(native.add(this.repr, other.repr));
  }

  round(scale: number, strategy: RoundingStrategy) {

  }

  toString(): string {
    return native.str(this.repr);
  }
}
