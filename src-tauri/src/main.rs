// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 设置 WebView2 用户数据目录到 exe 同级目录，实现便携化
    // 这样登录信息/cookie 保存在 exe 旁边，不占 C 盘
    #[cfg(target_os = "windows")]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                let data_dir = parent.join("MoonTV_Data");
                std::fs::create_dir_all(&data_dir).ok();
                std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &data_dir);
            }
        }
    }
    app_lib::run();
}