use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

/// Структура для представления точки
#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

/// Генерация случайного лабиринта методом рекурсивного деления
fn generate_maze(width: usize, height: usize) -> Vec<Vec<char>> {
    // Создаем пустое поле
    let mut maze = vec![vec!['_'; width]; height];
    
    // Добавляем границы
    for y in 0..height {
        maze[y][0] = '#';
        maze[y][width - 1] = '#';
    }
    for x in 0..width {
        maze[0][x] = '#';
        maze[height - 1][x] = '#';
    }
    
    // Рекурсивно делим пространство и добавляем стены
    divide_maze(&mut maze, 1, 1, width - 2, height - 2);
    
    // Добавляем старт и финиш
    let mut rng = thread_rng();
    
    // Выбираем случайные пустые места для старта и финиша
    let mut empty_cells = Vec::new();
    for y in 1..height-1 {
        for x in 1..width-1 {
            if maze[y][x] == '_' {
                empty_cells.push(Point { x, y });
            }
        }
    }
    
    if empty_cells.len() >= 2 {
        empty_cells.shuffle(&mut rng);
        
        // Устанавливаем старт
        let start = empty_cells[0];
        maze[start.y][start.x] = '1';
        
        // Устанавливаем финиш
        let end = empty_cells[1];
        maze[end.y][end.x] = 'F';
    }
    
    maze
}

/// Рекурсивное деление участка лабиринта
fn divide_maze(maze: &mut Vec<Vec<char>>, x: usize, y: usize, width: usize, height: usize) {
    if width < 2 || height < 2 {
        return;
    }
    
    let mut rng = thread_rng();
    
    // Выбираем случайно, горизонтальное или вертикальное деление
    let horizontal = if width < height {
        true
    } else if height < width {
        false
    } else {
        rng.gen_bool(0.5)
    };
    
    if horizontal {
        // Горизонтальное деление
        let wall_y = y + rng.gen_range(0..height);
        let hole_x = x + rng.gen_range(0..width);
        
        for i in x..x+width {
            if i != hole_x {
                maze[wall_y][i] = '#';
            }
        }
        
        // Рекурсивно делим верхнюю и нижнюю части
        divide_maze(maze, x, y, width, wall_y - y);
        divide_maze(maze, x, wall_y + 1, width, y + height - wall_y - 1);
    } else {
        // Вертикальное деление
        let wall_x = x + rng.gen_range(0..width);
        let hole_y = y + rng.gen_range(0..height);
        
        for i in y..y+height {
            if i != hole_y {
                maze[i][wall_x] = '#';
            }
        }
        
        // Рекурсивно делим левую и правую части
        divide_maze(maze, x, y, wall_x - x, height);
        divide_maze(maze, wall_x + 1, y, x + width - wall_x - 1, height);
    }
}

/// Сохранение лабиринта в файл
fn save_maze_to_file<P: AsRef<Path>>(maze: &Vec<Vec<char>>, path: P) -> std::io::Result<()> {
    let width = maze[0].len();
    let height = maze.len();
    
    let mut file = File::create(path)?;
    
    // Записываем размеры
    writeln!(&mut file, "{}", width)?;
    writeln!(&mut file, "{}", height)?;
    
    // Записываем сам лабиринт
    for row in maze {
        for &cell in row {
            write!(&mut file, "{}", cell)?;
        }
        writeln!(&mut file)?;
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Получаем аргументы командной строки или устанавливаем значения по умолчанию
    let args: Vec<String> = env::args().collect();
    
    let width = if args.len() > 1 {
        args[1].parse().unwrap_or(20)
    } else {
        20
    };
    
    let height = if args.len() > 2 {
        args[2].parse().unwrap_or(20)
    } else {
        20
    };
    
    let output_path = if args.len() > 3 {
        args[3].clone()
    } else {
        "examples/maze.txt".to_string()
    };
    
    println!("Генерация лабиринта {}x{}...", width, height);
    let maze = generate_maze(width, height);
    
    println!("Сохранение в файл {}...", output_path);
    save_maze_to_file(&maze, &output_path)?;
    
    println!("Готово!");
    Ok(())
}
