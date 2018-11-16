
function invoke(command) {
  window.external.invoke(JSON.stringify(command));
}

function log(...args) {
  const text = args.map(i => JSON.stringify(args, "", 4)).join("\n");
  invoke({ cmd: "log", text });
}

function addEventListener(event, callback) {

}

setInterval(() => {

}, 1);

export { invoke, log };
