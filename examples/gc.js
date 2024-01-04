// Run `cargo run --example gc` before running this
// Usage: `deno run --allow-all ./examples/gc.js <integer> <integer>`

const a = parseInt(Deno.args[0]);
const b = parseInt(Deno.args[1]);

const file = Deno.readFileSync("gc.wasm");
const m = new WebAssembly.Module(file);
const instance = new WebAssembly.Instance(m, {});

const arr = instance.exports.init(a, b);
console.log(instance.exports.sum(arr));
