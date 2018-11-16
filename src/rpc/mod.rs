
use web_view::{WebView, MyUnique};

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Status {
    ok,
    error,
    init,
    download,
    unzip,
    install,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    init,
    log { text: String },
}

#[derive(Deserialize)]
struct StateItem {}

pub fn init_callback(webview: MyUnique<WebView<Vec<Task>>>) {

}

pub fn rpc_exec_callback<'a, T>(webview: &mut WebView<'a, T>, arg: &str, state: &mut Vec<StateItem>) {
    match serde_json::from_str(arg).unwrap() {
        Cmd::init => (),
        Cmd::log { text } => println!("{}", text),
    }
}

pub fn init<'a, T>(js_code: &str, webview: &mut WebView<'a, T>) {
    
}

pub fn call_js<'a, T>(js_code: &str, webview: &mut WebView<'a, T>) {
    webview.eval(js_code);
}

pub fn dispatch_to_render<'a, T>(event: &str, arg: &str, webview: &mut WebView<'a, T>) {
    let code = format!("window.external.dispatch({}, {})", event, arg);
    call_js(&code, webview);
}
