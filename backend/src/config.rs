// use std::env;

// /// 初期化関数。dotenv を読み込む
// pub fn init() {
//     dotenv::dotenv().ok();
// }

// /// DATABASE_URL を取得
// pub fn database_url() -> String {
//     env::var("DATABASE_URL").expect("DATABASE_URL must be set")
// }

// /// Pythonスクリプトのパスを取得
// pub fn python_script_path() -> String {
//     env::var("PYTHON_SCRIPT_PATH").unwrap_or_else(|_| {
//         eprintln!("PYTHON_SCRIPT_PATH not set, using default");
//         "scripts/calculate_relevance.py".to_string()
//     })
// }