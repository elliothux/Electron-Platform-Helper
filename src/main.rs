#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;
extern crate os_info;

use web_view::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod utils;


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
  use Cmd::*;
  match serde_json::from_str(arg).unwrap() {
    init => (),
    log { text } => println!("{}", text),
    addTask { name } => tasks.push(Task { name, done: false }),
    markTask { index, done } => tasks[index].done = done,
    clearDoneTasks => tasks.retain(|t| !t.done),
  }
  render(webview, tasks);
}

fn open_install_helper() {
  let html = utils::generate_html(
    vec![include_str!("view/css/styles.css")],
    vec![include_str!("view/js/picodom.js"), include_str!("view/js/app.js")],
  );

  let title = "EL DEMO";
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

fn main() {
  let mut is_platform_exist = false;
  if cfg!(target_os = "macos") {
    // TODO: version
    is_platform_exist = utils::is_runtime_exist(utils::Platform::DARWIN, "3.0.6")
  } else if cfg!(target_os = "windows") {
    if cfg!(target_pointer_width = "32") {
      // TODO: win32
    } else if cfg!(target_pointer_width = "64") {
      // TODO: win64
    }
  } else if cfg!(target_os = "linux") {
    if cfg!(target_pointer_width = "32") {
      // TODO: linux32
    } else if cfg!(target_pointer_width = "64") {
      // TODO: linux64
    }
  } else {
    panic!("Unsupported platform!")
  }

  if is_platform_exist {
    // TODO: OPEN APP
    utils::open_app_bin();
  } else {
    open_install_helper();
  }
}
