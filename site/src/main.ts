import { compile_source } from "webrick";

const binary = compile_source("1 + 2 * 3");
WebAssembly.instantiate(binary, {}).then((x) =>
  console.log(x.instance.exports.main())
);
