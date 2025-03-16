use std::collections::{VecDeque, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn find_path(maze: &Vec<Vec<char>>, width: usize, height: usize) -> Option<Vec<Point>> {
    let mut start = None;
    let mut end = None;
    
    for y in 0..height {
        for x in 0..width {
            match maze[y][x] {
                '1' => start = Some(Point { x, y }),
                'F' => end = Some(Point { x, y }),
                _ => {}
            }
        }
    }
    
    let (start, end) = match (start, end) {
        (Some(s), Some(e)) => (s, e),
        _ => return None,
    };
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = vec![vec![None; width]; height];
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        if current == end {
            let mut path = Vec::new();
            let mut cur = Some(current);
            while let Some(p) = cur {
                path.push(p);
                cur = parent[p.y][p.x];
            }
            path.reverse();
            return Some(path);
        }
        
        for &(dx, dy) in &directions {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;
            
            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < width && ny < height && !visited.contains(&Point { x: nx, y: ny }) && maze[ny][nx] != '#' {
                    queue.push_back(Point { x: nx, y: ny });
                    visited.insert(Point { x: nx, y: ny });
                    parent[ny][nx] = Some(current);
                }
            }
        }
    }
    None
}

fn main() {
    let file = File::open("maze.txt").expect("Не удалось открыть файл");
    let mut lines = io::BufReader::new(file).lines();
    
    let width: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let height: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let maze: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
    
    if let Some(path) = find_path(&maze, width, height) {
        for point in path {
            println!("x:{}, y:{}", point.x + 1, point.y + 1);
        }
    } else {
        println!("Прохода нет");
    }
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
        assert_eq!(path.len(), 9); // Длина пути
        assert_eq!(path[0], Point { x: 0, y: 1 }); // Старт
        assert_eq!(path[8], Point { x: 1, y: 4 }); // Финиш
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
