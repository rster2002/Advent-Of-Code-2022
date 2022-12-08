use std::{env, fs};
use std::fmt::{Display, Formatter};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut forest = Forest::from_text_grid(file_content);
    mark_horizontal_lines(&mut forest);
    mark_vertical_lines(&mut forest);

    println!("{}", forest);

    let visible_trees = forest.get_trees()
        .iter()
        .filter(|tree| tree.visible)
        .count();

    println!("Visible trees: {}", visible_trees);
}

fn mark_horizontal_lines(forest: &mut Forest) {
    let mut y = 0;
    while y < forest.get_height() {
        let mut line = forest.get_horizontal_coords(y);
        mark_line(forest, &line);
        line.reverse();
        mark_line(forest, &line);

        y += 1;
    }
}

fn mark_vertical_lines(forest: &mut Forest) {
    let mut x = 0;
    while x < forest.get_width() {
        let mut line = forest.get_vertical_coords(x);
        mark_line(forest, &line);
        line.reverse();
        mark_line(forest, &line);

        x += 1;
    }
}

fn mark_line(forest: &mut Forest, coords: &Vec<(usize, usize)>) {
    let mut lowest: i32 = -1;

    for coord in coords {
        let height = forest.get_tree(coord.0, coord.1)
            .height;

        if height as i32 > lowest {
            forest.mark_as_visible(coord.0, coord.1);
            lowest = height as i32;
        }
    }
}

#[derive(Debug)]
struct Forest {
    width: usize,
    trees: Vec<Tree>,
}

impl Display for Forest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut y = 0;
        let mut lines = vec![];

        while y < self.get_height() {
            let string: String = self.get_horizontal_line(y)
                .iter()
                .map(|tree| {
                    if tree.visible {
                        'X'
                    } else {
                        '.'
                    }
                })
                .collect();

            lines.push(string);

            y += 1;
        }

        let completed_string = lines.join("\n");
        write!(f, "{}", completed_string)
    }
}

impl Forest {
    pub fn from_text_grid(grid: impl Into<String>) -> Self {
        let mut instance = Self {
            width: 0,
            trees: vec![],
        };

        let grid_string = grid.into();
        let lines = grid_string.lines().enumerate();
        for (y, line) in lines {
            instance.width = line.len();

            let chars = line.chars().enumerate();
            for (x, char) in chars {
                let height = char.to_string().parse().unwrap();
                let tree = Tree::new(x, y, height);

                instance.trees.push(tree);
            }
        }

        instance
    }

    pub fn get_trees(&self) -> Vec<&Tree> {
        self.trees
            .iter()
            .collect()
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.trees.len() / self.width
    }

    fn index_for_coords(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_mut_tree(&mut self, x: usize, y: usize) -> &mut Tree {
        let index = self.index_for_coords(x, y);
        self.trees.get_mut(index).unwrap()
    }

    pub fn get_tree(&self, x: usize, y: usize) -> &Tree {
        let index = self.index_for_coords(x, y);
        self.trees.get(index).unwrap()
    }

    pub fn get_horizontal_coords(&self, y: usize) -> Vec<(usize, usize)> {
        self.get_horizontal_line(y)
            .iter()
            .map(|tree| (tree.x, tree.y))
            .collect()
    }

    pub fn get_vertical_coords(&self, y: usize) -> Vec<(usize, usize)> {
        self.get_vertical_line(y)
            .iter()
            .map(|tree| (tree.x, tree.y))
            .collect()
    }

    pub fn get_horizontal_line(&self, y: usize) -> Vec<&Tree> {
        let mut result = vec![];
        let mut x: usize = 0;

        while x < self.get_width() {
            result.push(self.get_tree(x, y));
            x += 1;
        }

        result
    }

    pub fn get_vertical_line(&self, x: usize) -> Vec<&Tree> {
        let mut result = vec![];
        let mut y: usize = 0;

        while y < self.get_height() {
            result.push(self.get_tree(x, y));
            y += 1;
        }

        result
    }

    pub fn mark_as_visible(&mut self, x: usize, y: usize) {
        self.get_mut_tree(x, y).visible = true;
    }
}

#[derive(Debug, Default)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
    visible: bool,
}

impl Tree {
    pub fn new(x: usize, y: usize, height: u32) -> Self {
        Self {
            x,
            y,
            height,
            visible: false,
        }
    }
}
