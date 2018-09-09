use app::tetris_game::*;

#[cfg(test)]
pub trait TestableApp {
    fn test_inject_current_block(&mut self, block: &[u8; 16]);
    fn test_current_block(&self) -> [u8; 16];
    fn test_inject_grid_line(&mut self, y: usize, line: &[u8; 11]);
    fn test_current_grid(&self) -> [u8; 220];
    fn test_current_grid_line(&self, y: usize) -> [u8; 11];
}

#[cfg(test)]
impl TestableApp for TetrisGame {
    #[cfg(test)]
    fn test_inject_current_block(&mut self, block: &[u8; 16]) {
        for i in 0..16 {
            self.buffer[(self.width * self.height) + i] = block[i];
        }
    }

    #[cfg(test)]
    fn test_current_block(&self) -> [u8; 16] {
        let mut b: [u8; 16] = [0; 16];
        for n in 0..16 {
            let i = (self.width * self.height) + n;
            b[n] = self.buffer[i];
        }
        b
    }

    #[cfg(test)]
    fn test_inject_grid_line(&mut self, y: usize, line: &[u8; 11]) {
        for x in 0..self.width {
            self.buffer[(y * self.width) + x] = line[x];
        }
    }

    fn test_current_grid(&self) -> [u8; 220] {
        let mut r: [u8; 220] = [0; 220];
        let b: &[u8; 252] = &self.buffer;
        for n in 0..220 {
            r[n] = b[n];
        }
        r
    }

    fn test_current_grid_line(&self, y: usize) -> [u8; 11] {
        let mut r: [u8; 11] = [0; 11];
        for n in 0..11 {
            r[n] = self.buffer[(y * self.width) + n];
        }
        r
    }
}
