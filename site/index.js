const js = import("./node_modules/@honey-yasyuma/rust_test/rust_test.js");
js.then(js => {
  js.greet("WebAssembly");
});