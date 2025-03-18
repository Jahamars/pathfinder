#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::io::Write;

    // Вспомогательная функция для создания временного файла с лабиринтом
    fn create_test_maze(content: &str) -> String {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("test_maze.txt");
        
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        file_path.to_str().unwrap().to_string()
    }

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
    
    #[test]
    fn test_parse_maze_file() {
        let content = "5\n5\n#####\n1___#\n###_#\n#___#\n#F###\n";
        let file_path = create_test_maze(content);
        
        let (maze, width, height) = parse_maze_file(&file_path).unwrap();
        
        assert_eq!(width, 5);
        assert_eq!(height, 5);
        assert_eq!(maze[0][0], '#');
        assert_eq!(maze[1][0], '1');
        assert_eq!(maze[4][1], 'F');
    }
    
    #[test]
    fn test_find_start_end() {
        let maze = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['1', '_', '_', '_', '#'],
            vec!['#', '#', '#', '_', '#'],
            vec!['#', '_', '_', '_', '#'],
            vec!['#', 'F', '#', '#', '#'],
        ];
        
        let (start, end) = find_start_end(&maze, 5, 5).unwrap();
        
        assert_eq!(start, Point { x: 0, y: 1 });
        assert_eq!(end, Point { x: 1, y: 4 });
    }
    
    #[test]
    fn test_adjacent_cells() {
        let point = Point { x: 2, y: 2 };
        let maze = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['1', '_', '_', '_', '#'],
            vec!['#', '_', '_', '_', '#'],
            vec!['#', '_', '_', '_', '#'],
            vec!['#', 'F', '#', '#', '#'],
        ];
        
        let adjacent = get_adjacent_cells(&point, &maze, 5, 5);
        
        // Проверяем, что найдены все 4 соседние клетки
        assert_eq!(adjacent.len(), 4);
        assert!(adjacent.contains(&Point { x: 2, y: 1 }));
        assert!(adjacent.contains(&Point { x: 3, y: 2 }));
        assert!(adjacent.contains(&Point { x: 2, y: 3 }));
        assert!(adjacent.contains(&Point { x: 1, y: 2 }));
    }
    
    #[test]
    fn test_reconstruct_path() {
        let mut parent = vec![vec![None; 5]; 5];
        parent[2][2] = Some(Point { x: 2, y: 1 });
        parent[2][1] = Some(Point { x: 1, y: 1 });
        parent[1][1] = Some(Point { x: 0, y: 1 });
        
        let path = reconstruct_path(&parent, Point { x: 2, y: 2 }, Point { x: 0, y: 1 });
        
        assert_eq!(path.len(), 4);
        assert_eq!(path[0], Point { x: 0, y: 1 });
        assert_eq!(path[1], Point { x: 1, y: 1 });
        assert_eq!(path[2], Point { x: 2, y: 1 });
        assert_eq!(path[3], Point { x: 2, y: 2 });
    }
}
