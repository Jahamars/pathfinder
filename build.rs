use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    
    // Создаем директорию для тестовых данных, если она не существует
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_data_dir = Path::new(&out_dir).join("test_data");
    
    if !test_data_dir.exists() {
        fs::create_dir_all(&test_data_dir).unwrap();
        
        // Создаем тестовый файл лабиринта для тестов
        let test_maze_path = test_data_dir.join("test_maze.txt");
        fs::write(
            &test_maze_path,
            "5\n5\n#####\n1___#\n###_#\n#___#\n#F###\n",
        ).unwrap();
        
        println!("cargo:warning=Test data generated in: {}", test_data_dir.display());
    }
    
    // Определяем переменные окружения для различных сред
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-cfg=production");
    } else {
        println!("cargo:rustc-cfg=development");
    }
}
