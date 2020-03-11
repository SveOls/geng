#[derive(Clone, Debug)]
pub struct Rect(Vec<Vec<u32>>);

impl Rect {
    pub fn new(inp: Vec<Vec<u32>>) -> Rect {
        Rect(inp)
    }
    pub fn new_w(inp: Vec<u32>, w: usize) -> Rect {
        let mut ret = Vec::new();
        let mut temp = Vec::new();
        temp.reserve(w);
        for i in 0..inp.len() {
            temp.push(inp[i]);
            if (i + 1) % w == 0 {
                ret.push(temp.clone());
                println!("{:.x?}", temp);
                temp.clear();
            }
        }
        Rect(ret)
    }
}

impl super::Draw for Rect {
    fn draw(&self, inp: &mut [u32], width: usize, (x, y): (usize, usize)) {
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                inp[(i + y) * width + x + j] = self.0[i][j]
            }
        }
    }
}
