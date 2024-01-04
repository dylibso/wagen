const file = Deno.readFileSync("sum.wasm");
const m = new WebAssembly.Module(file);
const instance = new WebAssembly.Instance(m, {});

const a = Deno.args[0];
const b = Deno.args[1];
const arr = instance.exports.init(parseInt(a), parseInt(b));
// console.log(arr);
console.log(instance.exports.sum(arr));
