#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Level {
    floors: Vec<Vec<Vec<char>>>,
}

impl Level {
    fn new() -> Self {
        Self {
            floors: vec![vec![vec![]]],
        }
    }

    fn input_char(&mut self, pos: [usize; 3], c: char) {
        let mut max_index = self.floors.len();
        if pos[0] >= max_index {
            for _ in 0..=pos[0] - max_index {
                self.floors.push(vec![vec![]]);
            }
        }
        max_index = self.floors[pos[0]].len();
        if pos[1] >= max_index {
            for _ in 0..=pos[1] - max_index {
                self.floors[pos[0]].push(vec![]);
            }
        }
        max_index = self.floors[pos[0]][pos[1]].len();
        if pos[2] >= max_index {
            for _ in 0..=pos[2] - max_index {
                self.floors[pos[0]][pos[1]].push(' ');
            }
        }
        self.floors[pos[0]][pos[1]][pos[2]] = c;
    }
}

struct Cursor {
    pos: [i64; 3],
}

impl Cursor {
    pub fn move_dir(&mut self, dir: [i64; 3]) {
        self.pos[0] += dir[0];
        self.pos[1] += dir[1];
        self.pos[2] += dir[2];
        self.pos[0] = self.pos[0].max(0);
        self.pos[1] = self.pos[1].max(0);
        self.pos[2] = self.pos[2].max(0);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_char() {
        let mut level = Level::new();
        level.input_char([0, 1, 0], 'X');
        level.input_char([0, 0, 3], 'E');
        assert_eq!(
            level,
            Level {
                floors: vec![vec![vec![' ', ' ', ' ', 'E'], vec!['X']]]
            }
        );
        level.input_char([1, 0, 0], 'S');
        assert_eq!(
            level,
            Level {
                floors: vec![vec![vec![' ', ' ', ' ', 'E'], vec!['X']], vec![vec!['S']]]
            }
        );
    }

    #[test]
    fn test_cursor_movement() {
        let mut cursor = Cursor { pos: [0, 0, 0] };
        cursor.move_dir([0, 1, 0]);
        cursor.move_dir([-1, 0, 0]);
        cursor.move_dir([1, 1, 1]);
        cursor.move_dir([1, 0, -1]);
        cursor.move_dir([0, 0, -1]);
        cursor.move_dir([0, 0, 1]);
        assert_eq!(cursor.pos, [2, 2, 1]);
    }
}
