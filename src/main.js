(async () => {
  var importObject = {
    imports: {
      add: (a, b) => {
        console.log("> wasm call js: add(%d, %d)", a, b);
        return a + b;
      },
      hello: (sum) => {
        console.log("> wasm call js: hello(%d)", sum);
      },
    }
  };

  const bytes = require("fs").readFileSync('hello.wasm');
  const module = new WebAssembly.Module(bytes);
  const instance = await WebAssembly.instantiate(module, importObject);

  instance.exports.wasm_hello();
  const sum = instance.exports.wasm_add(2, 3);
  console.log('> js call wasm: add(2, 3) = %d', sum);
})();
