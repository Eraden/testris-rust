use app::tetris_game::*;

#[cfg(test)]
pub trait TestableApp {
    fn test_inject_current_block(&mut self, block: &[u8; 16]);
    fn test_current_block(&self) -> [u8; 16];
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
}
