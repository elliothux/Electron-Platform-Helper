
use web_view::*;
use serde_json;
use utils;


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    init,
    log { text: String },
    addTask { name: String },
    markTask { index: usize, done: bool },
    clearDoneTasks,
}

fn render<'a, T>(webview: &mut WebView<'a, T>, tasks: &[Task]) {
    println!("{:#?}", tasks);
    webview.eval(&format!("rpc.render({})", serde_json::to_string(tasks).unwrap()));
}

fn init_callback(webview: MyUnique<WebView<Vec<Task>>>) {
    //        webview.dispatch(|wv, _| wv.set_color(156, 39, 176, 255));
}

fn exec_callback<'a, T>(webview: &mut WebView<'a, T>, arg: &str, tasks: &mut Vec<Task>) {
    match serde_json::from_str(arg).unwrap() {
        Cmd::init => (),
        Cmd::log { text } => println!("{}", text),
        Cmd::addTask { name } => tasks.push(Task { name, done: false }),
        Cmd::markTask { index, done } => tasks[index].done = done,
        Cmd::clearDoneTasks => tasks.retain(|t| !t.done),
    }
    render(webview, tasks);
}

pub fn open_install_helper() {
    let html = utils::generate_html(
        vec![],
        vec![include_str!("../view/js/main.js")],
    );

    let title = "Electron Platform";
    let size = (800, 480);
    let resizable = true;
    let debug = true;

    let userdata = vec![];

    let (tasks, _) = run(
        title,
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        init_callback,
        exec_callback,
        userdata
    );
}
