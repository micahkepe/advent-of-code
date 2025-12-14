use std::error::Error;

fn x_mas_count(fp: &str) -> Result<u32, Box<dyn Error>> {
    let contents = std::fs::read_to_string(fp)?;
    let grid: Vec<Vec<char>> =
        contents.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Look for the center 'A' of the X-MAS pattern
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if grid[r][c] == 'A' {
                // Get the four corners
                let top_left = grid[r - 1][c - 1];
                let top_right = grid[r - 1][c + 1];
                let bottom_left = grid[r + 1][c - 1];
                let bottom_right = grid[r + 1][c + 1];

                // Check diagonal 1 (top-left to bottom-right): should be M-A-S or S-A-M
                let diag1_ok = (top_left == 'M' && bottom_right == 'S')
                    || (top_left == 'S' && bottom_right == 'M');

                // Check diagonal 2 (top-right to bottom-left): should be M-A-S or S-A-M
                let diag2_ok = (top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M');

                if diag1_ok && diag2_ok {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = x_mas_count("data/day_04_input.txt")?;
    println!("X-MAS count: {}", res);

    Ok(())
}
