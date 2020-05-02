const native = require("../native/index.node"); // eslint-disable-line

type NativeRepr = [number, ...number[]] & { length: 16 };

export enum RoundingStrategy {
  BANKERS,
  HALF_UP,
  HALF_DOWN,
}

export class Decimal {
  private constructor(private readonly repr: NativeRepr) {
  }

  static from(input: string): Decimal {
    return new Decimal(native.new(input));
  }

  add(other: Decimal): Decimal {
    return new Decimal(native.add(this.repr, other.repr));
  }

  sub(other: Decimal): Decimal {
    return new Decimal(native.sub(this.repr, other.repr));
  }

  mul(other: Decimal): Decimal {
    return new Decimal(native.mul(this.repr, other.repr));
  }

  round(dp: number, strategy: RoundingStrategy): Decimal {
    return new Decimal(native.round(this.repr, dp, strategy))
  }

  toString(): string {
    return native.str(this.repr);
  }
}
