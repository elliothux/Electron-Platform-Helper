 function initRPC() {
     Object.defineProperty(window, 'rpc', {
         writable: false,
         value: {}
     });

     function defineRPCProps(key, value) {
         Object.defineProperty(window.rpc, key, {
             writable: false,
             value: value
         });
     }

     [
         ['eventsMap', {}],
         ['addEventListener', function(event, callback) {
             if (eventsMap[event]) {
                 eventsMap[event].push(callback);
             } else {
                 eventsMap[event] = [callback];
             }
         }],
         ['removeEventListener', function(event, callback) {
             const callbacks = eventsMap[event];
             if (!callbacks) { return; }
             eventsMap[event] = callback.filter(i => i !== callback);
         }],
         ['removeAllEventListeners', function(event) {
             if (eventsMap[event]) {
                 delete eventsMap[event];
             }
         }],
         ['dispatch', function (event, arg) {
            var handlers = eventsMap[event];
            if (!handlers) { return; }
            handlers.forEach(i => i(arg));
         }],
         ['call', function(command, arg) {
            console.log(command);
             window.external.invoke(JSON.stringify(
                 Object.assign({ cmd: command }, command)
             ));
         }],
         ['log', function(...args) {
             const text = args.map(i => JSON.stringify(i, "", 4)).join("\n");
             window.rpc.call({ cmd: "log", text });
         }]
     ].map(i => defineRPCProps(i[0], i[1]));

     var eventsMap = window.rpc.eventsMap;
 }

 initRPC();
