
invoke({ cmd: 'init' });

function invoke(command) {
  window.external.invoke(JSON.stringify(command));
}

function log(...args) {
  const text = args.map(i => JSON.stringify(args, "", 4)).join("\n");
  invoke({ cmd: "log", text });
}

function addEventListener(event, callback) {
    if (eventsMap[event]) {
        eventsMap[event].push(callback);
    } else {
        eventsMap[event] = [callback];
    }
}

function removeEventListener(event, callback) {
    const callbacks = eventsMap[event];
    if (!callbacks) { return; }
    eventsMap[event] = callback.filter(i => i !== callback);
}

function removeAllEventListeners(event) {
    if (eventsMap[event]) {
        delete eventsMap[event];
    }
}

const eventsMap = {};
const { eventPool } = window.rpc;

setInterval(() => {
    if (!eventPool.length) { return; }
    const [event, arg] = eventPool.shift();
    if (!eventsMap[event]) { return; }
    eventsMap[event](arg);
}, 100);

export {
    invoke,
    log,
    addEventListener,
    removeEventListener,
    removeAllEventListeners
};
