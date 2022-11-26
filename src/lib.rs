use std::cmp::min;

use rand::{thread_rng, Rng};
use tui::widgets::Widget;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn random(width: usize, height: usize) -> Self {
        let mut cells = vec![false; height * width];
        thread_rng().fill(&mut cells[..]);
        Self {
            width,
            height,
            cells,
        }
    }

    fn idx_to_coord(&self, i: usize) -> (usize, usize) {
        let x = i % self.width;
        let y = i / self.width;
        (x, y)
    }

    fn coord_to_idx(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(x + y * self.width)
        } else {
            None
        }
    }

    fn count_living<I>(&self, indices: I) -> usize
    where
        I: Iterator<Item = usize>,
    {
        indices.filter(|&i| self.cells[i]).count()
    }

    fn count_living_neighbours(&self, i: usize) -> usize {
        let inc = |n: usize| n.checked_add(1);
        let dec = |n: usize| n.checked_sub(1);

        let (x, y) = self.idx_to_coord(i);

        // Cell X has 8 neighbours:
        //   1 2 3
        //   8 X 4
        //   7 6 5
        let neighbours = [
            (dec(x), inc(y)),  // 1
            (Some(x), inc(y)), // 2
            (inc(x), inc(y)),  // 3
            (inc(x), Some(y)), // 4
            (inc(x), dec(y)),  // 5
            (Some(x), dec(y)), // 6
            (dec(x), dec(y)),  // 7
            (dec(x), Some(y)), // 8
        ];

        let valid_neighbours = neighbours
            .into_iter()
            // Filter out coordinates for which x or y overflowed/underflowed.
            // These coordinates are off the grid and can be ignored.
            .filter_map(|(x, y)| Some((x?, y?)))
            // Convert coordinates to linear indicies, filtering out those which
            // could not be converted because they are located off the grid.
            .filter_map(|(x, y)| self.coord_to_idx(x, y));

        self.count_living(valid_neighbours)
    }

    fn next_at(&self, i: usize) -> bool {
        matches!(
            (self.cells[i], self.count_living_neighbours(i)),
            (true, 2 | 3) | (false, 3)
        )
    }

    pub fn update(&mut self) {
        let mut next_cells = vec![false; self.cells.len()];

        for (i, cell) in next_cells.iter_mut().enumerate() {
            *cell = self.next_at(i);
        }

        self.cells = next_cells;
    }
}

impl Widget for &Grid {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let height = min(self.height as u16, area.height);
        let width = min(self.width as u16, area.width);

        for y in 0..height {
            for x in 0..width {
                if let Some(i) = self.coord_to_idx(x as usize, y as usize) {
                    buf.get_mut(x, y)
                        .set_symbol(if self.cells[i] { "X" } else { " " });
                }
            }
        }
    }
}
