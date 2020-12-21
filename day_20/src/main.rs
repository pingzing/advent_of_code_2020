use array2d::Array2D;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self},
};
use std::{fs::File, io::Read};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_20/input.txt";
lazy_static! {
    static ref MONSTER_OFFSETS: Vec<(i32, i32)> = vec![
        (0, 0),
        (1, 0),
        (1, 1),
        (1, -1),
        (1, -6),
        (1, -7),
        (1, -12),
        (1, -13),
        (1, -18),
        (2, -2),
        (2, -5),
        (2, -8),
        (2, -11),
        (2, -14),
        (2, -17)
    ];
}

fn main() {
    let tiles = parse_input();
    let corner_ids = get_corners(&tiles);
    println!(
        "PART ONE: product of corner IDs is: {}",
        corner_ids.iter().product::<u64>()
    );

    let non_monster_water = part_two(&tiles);
    println!(
        "PART TWO: Non-monster filled cells count is: {}",
        non_monster_water
    );
}

fn get_corners(tiles: &[Tile]) -> Vec<u64> {
    let mut corner_ids = vec![];
    let mut reversed_scratch: Vec<&Cell> = vec![];

    for tile in tiles.iter() {
        let mut matches_counts = 0;
        let mut edges: Vec<Vec<&Cell>> = vec![];
        edges.push(tile.contents.column_iter(0).collect());
        edges.push(tile.contents.column_iter(9).collect());
        edges.push(tile.contents.row_iter(0).collect());
        edges.push(tile.contents.row_iter(9).collect());

        for other_tile in tiles.iter().filter(|x| *x != tile) {
            let mut other_edges: Vec<Vec<&Cell>> = vec![];
            other_edges.push(other_tile.contents.column_iter(0).collect());
            other_edges.push(other_tile.contents.column_iter(9).collect());
            other_edges.push(other_tile.contents.row_iter(0).collect());
            other_edges.push(other_tile.contents.row_iter(9).collect());

            for tile_edge in edges.iter() {
                for other_edge in other_edges.iter() {
                    if edges_equal_as_ref(tile_edge, other_edge) {
                        matches_counts += 1;
                    }
                    // Try it with other_edge reversed, too
                    reversed_scratch.truncate(0);
                    reversed_scratch.extend(other_edge.iter().rev());
                    if edges_equal_as_ref(tile_edge, &reversed_scratch) {
                        matches_counts += 1;
                    }
                }
            }
        }

        if matches_counts == 2 {
            corner_ids.push(tile.id as u64);
        }
    }

    println!("Corner Ids: {:?}", corner_ids);

    corner_ids
}

fn part_two(tiles: &[Tile]) -> u32 {
    //--- Arrange tiles so that all borders are lined up ---

    // Corner:
    // - Top left: 2273, rotated twice, discovered manually

    let mut haystack = tiles.to_vec();
    let mut arranged_tiles = vec![];
    let mut image_width = 0usize;
    let mut image_height = 1usize;

    let top_left_index = haystack.iter().position(|x| x.id == 2273).unwrap();
    let mut top_left = haystack.get(top_left_index).unwrap().clone();

    top_left.rotate_right();
    top_left.rotate_right();

    arranged_tiles.push(top_left);
    haystack.remove(top_left_index);

    let mut col_counter = 1usize;
    let mut curr_leftmost_index = 0usize;
    let mut curr_rightmost_index = 0usize;

    while !haystack.is_empty() {
        // Walk right, looking for matching borders until we don't find any anymore. Them go down a row and repeat
        let rightmost_tile = arranged_tiles.get(curr_rightmost_index).unwrap();
        let rightmost_column: Vec<&Cell> = rightmost_tile.contents.column_iter(9).collect();
        let matched_tile = get_matched_tile(&rightmost_column, &haystack, TileSide::Left);
        if let Some(matched_tile) = matched_tile {
            let matched_index = haystack
                .iter()
                .position(|x| x.id == matched_tile.id)
                .unwrap();
            arranged_tiles.push(matched_tile);
            haystack.remove(matched_index);
            curr_rightmost_index = arranged_tiles.len() - 1;
            col_counter += 1;
        } else {
            // We got a none, which means we need to go back to our current-leftmost tile, then down
            // Get bottom border of leftmost
            if image_width == 0 {
                image_width = col_counter;
            }
            let leftmost_bottom: Vec<&Cell> = arranged_tiles
                .get(curr_leftmost_index)
                .unwrap()
                .contents
                .row_iter(9)
                .collect();
            let matched_tile = get_matched_tile(&leftmost_bottom, &haystack, TileSide::Top);
            if let Some(matched_tile) = matched_tile {
                let matched_index = haystack
                    .iter()
                    .position(|x| x.id == matched_tile.id)
                    .unwrap();
                arranged_tiles.push(matched_tile);
                haystack.remove(matched_index);
                curr_leftmost_index = arranged_tiles.len() - 1;
                curr_rightmost_index = arranged_tiles.len() - 1;
                image_height += 1;
            } else {
                panic!("Tried to go down, but couldn't find any matching borders. arranged_tiles len is: {}, haystack len is {}", arranged_tiles.len(), haystack.len());
            }
        }
    }

    println!(
        "Done running work so far, arranged_tiles len is: {}, haystack len is {}, image width is: {}",
        arranged_tiles.len(),
        haystack.len(),
        image_width
    );

    // --- Remove borders and stitch tiles together ---

    // Remove borders
    for tile in arranged_tiles.iter_mut() {
        let mut borderless_tile = vec![];
        for row in 1..tile.contents.column_len() - 1 {
            for col in 1..tile.contents.row_len() - 1 {
                borderless_tile.push(tile.contents[(row, col)]);
            }
        }
        tile.contents = Array2D::from_row_major(&borderless_tile, 8, 8);
    }

    println!(
        "First three tiles:\nTile: {}\n{}\nTile:{}\n{}\nTile:{}\n{}",
        arranged_tiles[0].id,
        arranged_tiles[0],
        arranged_tiles[1].id,
        arranged_tiles[1],
        arranged_tiles[2].id,
        arranged_tiles[2]
    );

    let tile_height = 8;
    let tile_width = 8;
    let tile_row_height = arranged_tiles[0].contents.column_len();
    let mut row_major_image_cells = vec![];
    for row_of_tiles in arranged_tiles.chunks_exact(image_width) {
        for row_idx in 0..tile_row_height {
            for tile in row_of_tiles {
                for cell in tile.contents.row_iter(row_idx) {
                    row_major_image_cells.push(*cell);
                }
            }
        }
    }

    let stitched_image = Array2D::from_row_major(
        &row_major_image_cells,
        tile_height * image_height,
        tile_width * image_width,
    );
    let mut giant_tile = Tile {
        id: 0,
        contents: stitched_image,
    };

    let mut sea_monsters = HashSet::new();
    for _ in 0..4 {
        for _ in 0..4 {
            sea_monsters = find_sea_monsters(&giant_tile.contents);
            if !sea_monsters.is_empty() {
                break;
            }
            giant_tile.rotate_right();
        }
        if !sea_monsters.is_empty() {
            break;
        }
        giant_tile.flip_vertically();
    }

    println!("Correctly-oriented giant tile:\n{}", giant_tile);

    // mark all the sea monsters for debugging and sanity-checking
    for (row, col) in sea_monsters.iter() {
        giant_tile.contents[(*row, *col)] = Cell::SeaMonster;
    }

    println!(
        "Correctly-oriented giant tile with MONSTERS!:\n{}",
        giant_tile
    );

    // We've found our sea monsters! Count up our non-monster Filled cells
    giant_tile
        .contents
        .elements_row_major_iter()
        .filter(|x| **x == Cell::Filled)
        .count() as u32
}

fn find_sea_monsters(image: &Array2D<Cell>) -> HashSet<(usize, usize)> {
    // Sea monster pattern (20 wide, 3 tall)
    //                  #
    //#    ##    ##    ###
    // #  #  #  #  #  #
    // vanguard (0, 0)
    // 1 down and (0, +1, -1, -6, -7, -12, -13, -18) to the side
    // 2 down and (-2, -5, -8, -11, -14, -17)
    let mut sea_monster_cells = HashSet::new();
    let image_height = image.column_len();
    let image_width = image.row_len();
    for row_i in 0..image_height - 2 {
        // -2 because a sea monster is always 3 tall--we'll never find one in the bottom two rows
        for col_i in 0..image_width {
            if col_i < 18 || col_i == image_width - 1 {
                // the vanguard must be at least 18 away from the left edge and 1 away from the right edge
                continue;
            }
            // check to see if vanguard has found a monster head
            let vanguard = image[(row_i, col_i)];
            if vanguard == Cell::Filled {
                // if it has, check the other cells that need to be filled with monster bits
                let candidate_cells: Vec<(usize, usize)> = MONSTER_OFFSETS
                    .iter()
                    .map(|x| ((row_i as i32 + x.0) as usize, (col_i as i32 + x.1) as usize))
                    .collect();

                //println!("Checking: {:?}", other_cells.iter());

                // make sure none of the possible cells are already in our hashset
                if candidate_cells
                    .iter()
                    .any(|x| sea_monster_cells.contains(x))
                {
                    continue;
                }
                // If not, we may have found a new monster. make sure that all the cells are filled
                if candidate_cells
                    .iter()
                    .all(|x| image[(x.0, x.1)] == Cell::Filled)
                {
                    // they are. record that sucker
                    sea_monster_cells.extend(candidate_cells.iter());
                }
            }
        }
    }

    sea_monster_cells
}

fn get_matched_tile(border: &[&Cell], other_tiles: &[Tile], look_at: TileSide) -> Option<Tile> {
    for other_tile in other_tiles {
        let mut other_tile = other_tile.clone();
        let mut other_border: Vec<Cell>;
        for _ in 0..4 {
            // all four flips
            for _ in 0..4 {
                // all four rotations
                other_border = get_border(&other_tile, &look_at);
                if edges_equal(border, &other_border) {
                    return Some(other_tile);
                }
                // no? rotate it and try again
                other_tile.rotate_right();
            }

            // still no? flip and try all four rotations again
            other_tile.flip_vertically();
        }
    }

    None
}

fn get_border(tile: &Tile, look_at: &TileSide) -> Vec<Cell> {
    match look_at {
        TileSide::Left => tile.contents.column_iter(0).cloned().collect::<Vec<Cell>>(),
        TileSide::Top => tile.contents.row_iter(0).cloned().collect::<Vec<Cell>>(),
    }
}

fn edges_equal_as_ref(lhs: &[&Cell], rhs: &[&Cell]) -> bool {
    lhs.iter().zip(rhs.iter()).all(|(l, r)| **l == **r)
}

fn edges_equal(lhs: &[&Cell], rhs: &[Cell]) -> bool {
    lhs.iter().zip(rhs.iter()).all(|(l, r)| **l == *r)
}

fn parse_input() -> Vec<Tile> {
    let file = File::open(PATH).unwrap();
    let mut input_string = String::new();
    io::BufReader::new(file)
        .read_to_string(&mut input_string)
        .unwrap();
    let tile_strings = input_string.split("\r\n\r\n");

    tile_strings
        .map(|string| {
            let mut lines = string.lines();
            let id_line = lines.next().unwrap();
            let id = id_line
                .chars()
                .skip(5)
                .collect::<String>()
                .replace(':', "")
                .parse()
                .unwrap();
            let cells_iter = lines.map(|x| x.chars()).flatten().map(|x| match x {
                '#' => Cell::Filled,
                '.' => Cell::Empty,
                _ => panic!("Unexpected cell value"),
            });
            let cell_matrix = Array2D::from_iter_row_major(cells_iter, 10, 10);
            Tile {
                id,
                contents: cell_matrix,
            }
        })
        .collect()
}

#[derive(Debug, Eq, Clone)]
struct Tile {
    id: u32,
    contents: Array2D<Cell>,
}

impl Tile {
    fn flip_vertically(&mut self) {
        for col in 0..self.contents.row_len() {
            let mut start = 0usize;
            let mut end = self.contents.column_len() - 1;
            while start < end {
                let temp = self.contents[(start, col)];
                self.contents[(start, col)] = self.contents[(end, col)];
                self.contents[(end, col)] = temp;
                start += 1;
                end -= 1;
            }
        }
    }

    fn rotate_right(&mut self) {
        self.transpose();
        for row in 0..self.contents.column_len() {
            let mut start = 0usize;
            let mut end = self.contents.row_len() - 1;
            while start < end {
                let temp = self.contents[(row, start)];
                self.contents[(row, start)] = self.contents[(row, end)];
                self.contents[(row, end)] = temp;
                start += 1;
                end -= 1;
            }
        }
    }

    fn transpose(&mut self) {
        for row in 0..self.contents.column_len() {
            for col in row..self.contents.row_len() {
                let temp = self.contents[(row, col)];
                self.contents[(row, col)] = self.contents[(col, row)];
                self.contents[(col, row)] = temp;
            }
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.contents.rows_iter() {
            for cell in row {
                write!(f, "{}", *cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty,
    Filled,
    SeaMonster,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Filled => write!(f, "#"),
            Cell::SeaMonster => write!(f, "O"),
        }
    }
}

enum TileSide {
    Left,
    Top,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Filled => write!(f, "#"),
            Cell::SeaMonster => write!(f, "O"),
        }
    }
}
