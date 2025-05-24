// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_two_numbers(num1: i64, num2: i64) -> String {
    format!("{}", num1 + num2)
}

/*
#[tauri::command]
fn add_two_numbers(num1: &str, num2: &str) -> String {
    let result: Result<i32, _> = num1.parse();
    let num1_num = if let Ok(num) = result {
        num
    } else {
        panic!()
    };

    let result: Result<i32, _> = num2.parse();
    let num2_num = if let Ok(num) = result {
        num
    } else {
        panic!()
    };
       
    format!("{}", num1_num + num2_num)
}
// */

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_two_numbers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
