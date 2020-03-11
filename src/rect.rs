use bmp;

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
                temp.clear();
            }
        }
        Rect(ret)
    }
    pub fn new_outline(&self) -> Rect {
        let w = self.0[0].len();
        let h = self.0.len();
        let mut ret = Vec::with_capacity(h);
        ret.push(vec![0xFF000000; w]);
        let mut temp = vec![0xFF000000];
        temp.append(&mut vec![0xFFFFFF; w - 2]);
        temp.push(0xFF000000);
        for _ in 2..h {
            ret.push(temp.clone());
        }
        ret.push(vec![0xFF000000; w]);
        Rect(ret)
    }
    pub fn from_bmp(name: &str) -> Result<Rect, Box<dyn std::error::Error>> {
        let a = bmp::open(name)?;
        let mut ret = Vec::new();
        let w = a.get_width() as usize;
        let mut pix: bmp::Pixel;
        for (x, y) in a.coordinates() {
            pix = a.get_pixel(x, y);
            ret.push(0xFF000000 + (pix.r as u32) * 0x10000 + (pix.g as u32) * 0x100 + (pix.b as u32));
        }
        Ok(Rect::new_w(ret, w))
    }
}

impl super::Draw for Rect {
    fn draw(&self, inp: &mut [u32], width: usize, (x, y): (usize, usize)) {
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                // implement opacity!!!!
                if self.0[i][j] / 0x1000000 != 0 {
                    if let Some(a) = inp.get_mut((i + y) * width + x + j) {
                        *a = self.0[i][j];
                    }
                }
            }
        }
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn contains(&self, inp: (usize, usize)) -> bool {
        if inp.0 < self.width() && inp.1 < self.height() {
            self.0[inp.1][inp.0] / 0x1000000 != 0
        } else {
            false
        }
    }
    fn get_outline(&self) -> Box<dyn super::Draw> {
        Box::new(self.new_outline())
    }
}
