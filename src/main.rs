use clap::{Parser, ValueEnum};
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Решение задачи о поиске пути в лабиринте")]
struct Args {
    /// Путь к файлу с лабиринтом
    #[arg(short, long, default_value = "maze.txt")]
    path: String,

    /// Формат вывода
    #[arg(short, long, value_enum, default_value_t=OutputFormat::Text)]
    format: OutputFormat,

    /// Режим отладки
    #[arg(short, long)]
    debug: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Текстовый формат (координаты точек)
    Text,
    /// Визуализация лабиринта с путём
    Visual,
    /// JSON формат
    Json,
}

/// Парсинг файла с лабиринтом
fn parse_maze_file<P: AsRef<Path>>(path: P) -> io::Result<(Vec<Vec<char>>, usize, usize)> {
    let file = File::open(path)?;
    let mut lines = io::BufReader::new(file).lines();

    let width: usize = lines
        .next()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Отсутствует информация о ширине",
            )
        })?
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Некорректная ширина"))?;

    let height: usize = lines
        .next()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Отсутствует информация о высоте",
            )
        })?
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Некорректная высота"))?;

    let maze: Vec<Vec<char>> = lines
        .map(|line| line.map(|l| l.chars().collect()))
        .collect::<Result<Vec<Vec<char>>, _>>()?;

    // Проверяем, что размеры лабиринта совпадают с указанными
    if maze.len() != height || maze.iter().any(|row| row.len() != width) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Размеры лабиринта не соответствуют указанным",
        ));
    }

    Ok((maze, width, height))
}

/// Поиск координат старта и финиша
fn find_start_end(maze: &Vec<Vec<char>>, width: usize, height: usize) -> Option<(Point, Point)> {
    let mut start = None;
    let mut end = None;

    // Ищем координаты старта и финиша
    for y in 0..height {
        for x in 0..width {
            match maze[y][x] {
                '1' => start = Some(Point { x, y }),
                'F' => end = Some(Point { x, y }),
                _ => {}
            }
        }
    }

    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}

/// Получение соседних ячеек
fn get_adjacent_cells(
    point: &Point,
    maze: &Vec<Vec<char>>,
    width: usize,
    height: usize,
) -> Vec<Point> {
    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut adjacent = Vec::new();

    for &(dx, dy) in &directions {
        let (nx, ny) = (point.x.wrapping_add(dx), point.y.wrapping_add(dy));
        if nx < width && ny < height && maze[ny][nx] != '#' {
            adjacent.push(Point { x: nx, y: ny });
        }
    }

    adjacent
}

/// Восстановление пути из родительских указателей
fn reconstruct_path(parent: &Vec<Vec<Option<Point>>>, end: Point, start: Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut current = end;

    while current != start {
        path.push(current);
        current = parent[current.y][current.x].unwrap();
    }

    path.push(start);
    path.reverse();

    path
}

/// Ищем кратчайший путь от '1' (старта) к 'F' (финишу) в лабиринте
pub fn find_path(maze: &Vec<Vec<char>>, width: usize, height: usize) -> Option<Vec<Point>> {
    let (start, end) = find_start_end(maze, width, height)?;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = vec![vec![None; width]; height];

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            return Some(reconstruct_path(&parent, end, start));
        }

        for next in get_adjacent_cells(&current, maze, width, height) {
            if !visited.contains(&next) {
                queue.push_back(next);
                visited.insert(next);
                parent[next.y][next.x] = Some(current);
            }
        }
    }

    None
}

/// Визуализация лабиринта с найденным путём
fn visualize_maze(maze: &Vec<Vec<char>>, path: &Vec<Point>) {
    let mut visual_maze = maze.clone();

    // Отмечаем путь символом '*', кроме начала и конца
    for &point in path.iter().skip(1).rev().skip(1) {
        visual_maze[point.y][point.x] = '*';
    }

    // Выводим лабиринт
    for row in &visual_maze {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

/// Формирование JSON-представления результата
fn format_json(path: &Vec<Point>) -> String {
    let points: Vec<String> = path
        .iter()
        .map(|p| format!(r#"{{"x":{},"y":{}}}"#, p.x + 1, p.y + 1))
        .collect();

    format!(r#"{{"path":[{}]}}"#, points.join(","))
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Загружаем лабиринт из файла
    let (maze, width, height) = parse_maze_file(&args.path)?;

    // Поиск пути
    if let Some(path) = find_path(&maze, width, height) {
        match args.format {
            OutputFormat::Text => {
                // Вывод координат пути
                for point in path {
                    println!("x:{}, y:{}", point.x + 1, point.y + 1);
                }
            }
            OutputFormat::Visual => {
                // Визуализация лабиринта с путём
                visualize_maze(&maze, &path);
            }
            OutputFormat::Json => {
                // JSON-формат
                println!("{}", format_json(&path));
            }
        }
    } else {
        match args.format {
            OutputFormat::Json => println!(r#"{{"error":"Прохода нет"}}"#),
            _ => println!("Прохода нет"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_exists() {
        let maze = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['1', '_', '_', '_', '#'],
            vec!['#', '#', '#', '_', '#'],
            vec!['#', '_', '_', '_', '#'],
            vec!['#', 'F', '#', '#', '#'],
        ];
        let path = find_path(&maze, 5, 5).unwrap();
        assert_eq!(path.len(), 9);
        assert_eq!(path[0], Point { x: 0, y: 1 });
        assert_eq!(path[8], Point { x: 1, y: 4 });
    }

    #[test]
    fn test_no_path() {
        let maze = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['1', '_', '_', '_', '#'],
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '_', '_', '_', '#'],
            vec!['#', 'F', '#', '#', '#'],
        ];
        assert_eq!(find_path(&maze, 5, 5), None);
    }
}
