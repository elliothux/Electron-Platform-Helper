#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use web_view::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod utils;

fn main() {
    let mut html = String::new();
    let mut file = File::open(Path::new("./view/index.html")).expect("file not found");
    file.read_to_string(&mut html).expect("Unable to read the file");
    let size = (800, 480);
    let resizable = false;
    let debug = true;
    let init_cb = |webview: MyUnique<WebView<Vec<Task>>>| {
//        webview.dispatch(|wv, _| wv.set_color(156, 39, 176, 255));
    };
    let userdata = vec![];
    let (tasks, _) = run("Rust Todo App", Content::Html(html), Some(size), resizable, debug, init_cb, |webview, arg, tasks: &mut Vec<Task>| {
        use Cmd::*;
        match serde_json::from_str(arg).unwrap() {
            init => (),
            log { text } => println!("{}", text),
            addTask { name } => tasks.push(Task { name, done: false }),
            markTask { index, done } => tasks[index].done = done,
            clearDoneTasks => tasks.retain(|t| !t.done),
        }
//        webview.set_title(&format!("Rust Todo App ({} Tasks)", tasks.len()));
        render(webview, tasks);
    }, userdata);
    println!("final state: {:?}", tasks);
}

fn render<'a, T>(webview: &mut WebView<'a, T>, tasks: &[Task]) {
    println!("{:#?}", tasks);
    webview.eval(&format!("rpc.render({})", serde_json::to_string(tasks).unwrap()));
}

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
