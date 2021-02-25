// For more comments about what's going on here, check out the `hello_world`
// example
const wasm = import('./pkg');
// wasm
//   .then(m => alert('1 + 2 = ' + m.add_u32(1, 2)))
//   .catch(console.error);

// wasm
//   .then(m => alert('192233720368547758n + 392233720368547758n = ' + m.add_u64(192233720368547758n, 392233720368547758n)))
//   .catch(console.error);
const ft = wasm.create_foo_test(192233720368547758n, 392233720368547758n);
const r = wasm.foo_test_add(ft);

alert('footest = ' + r);