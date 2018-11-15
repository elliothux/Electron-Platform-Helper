
use unzip;
use web_view::*;
use serde_json;
use utils;
use helper;
use downloader;
use model::{Version};
use std::path::{PathBuf, Path};
use std::fs;
use std::process::Command;


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

pub fn install_runtime(v: Version) -> Result<(), String> {
    match unzip_runtime(v) {
        Err(why) => Err(why),
        Ok(unzip_path) => {
            let runtime_path = unzip_path.join("Electron.app/Contents/Frameworks");
            let target_path = helper::get_runtimes_path()
                .join(helper::version_to_string(v));
            let move_result = fs::rename(runtime_path, target_path);
            fs::remove_dir_all(&unzip_path);
            if let Ok(_) = move_result {
                Ok(())
            } else {
                Err("Move runtime files failed".to_owned())
            }
        }
    }

}

fn unzip_runtime(v: Version) -> Result<PathBuf, String> {
    let from = helper::get_platform_path()
        .join(format!("temp/{}.zip", helper::version_to_string(v)));
    let to = helper::get_platform_path()
        .join(format!("temp/{}", helper::version_to_string(v)));
    unzip_file(&from, to)
}

fn unzip_file(file_path: &PathBuf, to: PathBuf) -> Result<PathBuf, String> {
    if cfg!(target_os = "macos") {
        let result = Command::new("unzip")
            .args(&[
                "-n",
                file_path.to_str().unwrap(),
                "-d",
                to.to_str().unwrap()
            ])
            .output();
        match result {
            Ok(_) => Ok(to),
            Err(_) => Err("Failed to unzip file".to_owned())
        }
    } else {
        let file = fs::File::open(file_path).unwrap();
        let archive_result = unzip::Unzipper::new(file, &to).unzip();
        match archive_result {
            Ok(_) => Ok(to),
            Err(_) => Err("Failed to unzip file".to_owned())
        }
    }
}
