const decimal = require('../native/index.node');

const d1 = decimal.new("1.5");
const d2 = decimal.add(d1);
const str1 = decimal.str(d1);
const str2 = decimal.str(d2);

console.log(d1, d2, str1, str2);

module.exports = decimal;
