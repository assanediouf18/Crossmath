import { Crossmath } from "wasm-crossmath";

const crossmath = Crossmath.new(2);
const width = crossmath.width();
const height = crossmath.height();
console.log(width)
console.log(crossmath.render());