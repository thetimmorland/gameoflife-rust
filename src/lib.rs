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
        let neighbours = [
            (inc(x), inc(y)),
            (dec(x), dec(y)),
            (inc(x), dec(y)),
            (dec(x), inc(y)),
            (Some(x), inc(y)),
            (inc(x), Some(y)),
            (Some(x), dec(y)),
            (dec(x), Some(y)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.coord_to_idx(x?, y?));

        self.count_living(neighbours)
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
