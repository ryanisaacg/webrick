import { compile_source } from "webrick";

import { EditorView, basicSetup } from "codemirror";

let view = new EditorView({
  extensions: [basicSetup],
  parent: document.getElementById("editor")!,
});

const outputElement = document.getElementById("output")!;

document.getElementById("build")!.addEventListener("click", () => {
  const { binary, errors } = compile_source(view.state.doc.toString());
  if (binary != null) {
    WebAssembly.instantiate(binary, {}).then((x) => {
      outputElement.innerText = (x.instance.exports.main as CallableFunction)();
    });
  } else {
    outputElement.innerText =
      errors ?? "No output and no errors - this is probably an ICE";
  }
});
