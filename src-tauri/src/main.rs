#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]



fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      search_wal,
      wal_image,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn search_wal(query: String, page: u16) -> String {
  reqwest::get(format!("https://wallhaven.cc/api/v1/search?q={}&page={}", query, page))
    .await.unwrap()
    .text()
    .await.unwrap()
}

fn check_file_exists(path: &String) -> bool {
  use std::path::Path;
  Path::new(path).exists()
}

fn create_dl_folder() {
  use std::fs::create_dir;
  let dl_dir = format!("{}/.cache/wal-it", std::env::var("HOME").unwrap());

  create_dir(dl_dir);
}

#[tauri::command]
async fn wal_image(path: String) {
  use std::fs::File;
  use std::io::Write;

  let dl_dir = format!("{}/.cache/wal-it", std::env::var("HOME").unwrap());
  let filename = path.split("/").last().unwrap();
  let path_to_file = format!("{}/{}", dl_dir, filename);
  println!("{}", path_to_file);

  if !check_file_exists(&dl_dir.to_string()) {
    create_dl_folder();
  }

  if !check_file_exists(&path_to_file) {
    let data = reqwest::get(&path)
      .await.unwrap()
      .bytes()
      .await.unwrap();
    let mut file = match File::create(&path_to_file) {
        Err(why) => panic!("couldn't create: {}", why),
        Ok(file) => file,
      };
    file.write_all(&data).unwrap();
  }

  let res = std::process::Command::new("wal")
    .arg("-i")
    .arg(format!("{}/{}", dl_dir, filename))
    .output().unwrap();

  println!("{:#?}", res);
}