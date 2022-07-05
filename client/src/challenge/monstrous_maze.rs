use common::challenge::models_monstrous_maze::{MonstrousMazeInput, MonstrousMazeOutput};

pub fn maze_challenge_resolver(input: MonstrousMazeInput) -> MonstrousMazeOutput {
    let grid = input
        .grid
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    return MonstrousMazeOutput {
        path: find_start_and_end_of_maze(grid),
    };
}

fn find_start_and_end_of_maze(grid: Vec<Vec<char>>) -> String {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'I' {
                start_x = x;
                start_y = y;
            } else if *cell == 'X' {
                end_x = x;
                end_y = y;
            }
        }
    }
    let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];
    visited[start_y][start_x] = 0;
    let mut queue = Vec::new();
    queue.push((start_x, start_y));
    while !queue.is_empty() {
        let (x, y) = queue.pop().expect("failed to pop queue");
        if grid[y][x] == 'X' {
            break;
        }
        update_around_cell(&grid, &mut visited, x, y, &mut queue);
    }
    return find_path(&visited, end_x, end_y);
}

fn update_around_cell(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
    queue: &mut Vec<(usize, usize)>,
) {
    if (y > 0)
        && (grid[y - 1][x] != '#')
        && ((visited[y - 1][x] > (visited[y][x] + 1)) || (visited[y - 1][x] == -1))
    {
        visited[y - 1][x] = visited[y][x] + 1;
        queue.push((x, y - 1));
    }
    if y < grid.len() - 1
        && grid[y + 1][x] != '#'
        && ((visited[y + 1][x] > (visited[y][x] + 1)) || (visited[y + 1][x] == -1))
    {
        visited[y + 1][x] = visited[y][x] + 1;
        queue.push((x, y + 1));
    }
    if x > 0
        && grid[y][x - 1] != '#'
        && ((visited[y][x - 1] > (visited[y][x] + 1)) || (visited[y][x - 1] == -1))
    {
        visited[y][x - 1] = visited[y][x] + 1;
        queue.push((x - 1, y));
    }
    if x < grid[0].len() - 1
        && grid[y][x + 1] != '#'
        && ((visited[y][x + 1] > (visited[y][x] + 1)) || (visited[y][x + 1] == -1))
    {
        visited[y][x + 1] = visited[y][x] + 1;
        queue.push((x + 1, y));
    }
}

fn find_path(visited: &Vec<Vec<i32>>, end_x: usize, end_y: usize) -> String {
    let mut path = String::new();
    let mut x = end_x;
    let mut y = end_y;
    while visited[y][x] != 0 {
        let (new_x, new_y) = find_next_cell(visited, x, y);
        path.push(grid_to_char(new_x, new_y, x, y));
        x = new_x;
        y = new_y;
    }
    return path.chars().rev().collect::<String>();
}

fn find_next_cell(visited: &Vec<Vec<i32>>, x: usize, y: usize) -> (usize, usize) {
    let mut min_distance = std::i32::MAX;
    let mut min_x = 0;
    let mut min_y = 0;
    if y > 0 && visited[y - 1][x] < min_distance && visited[y - 1][x] != -1 {
        min_distance = visited[y - 1][x];
        min_x = x;
        min_y = y - 1;
    }
    if y < visited.len() - 1 && visited[y + 1][x] < min_distance && visited[y + 1][x] != -1 {
        min_distance = visited[y + 1][x];
        min_x = x;
        min_y = y + 1;
    }
    if x > 0 && visited[y][x - 1] < min_distance && visited[y][x - 1] != -1 {
        min_distance = visited[y][x - 1];
        min_x = x - 1;
        min_y = y;
    }
    if x < visited[0].len() - 1 && visited[y][x + 1] < min_distance && visited[y][x + 1] != -1 {
        min_x = x + 1;
        min_y = y;
    }
    (min_x, min_y)
}

fn grid_to_char(new_x: usize, new_y: usize, previous_x: usize, previous_y: usize) -> char {
    return if new_x < previous_x {
        '>'
    } else if new_x > previous_x {
        '<'
    } else if new_y < previous_y {
        'v'
    } else if new_y > previous_y {
        '^'
    } else {
        ' '
    };
}
