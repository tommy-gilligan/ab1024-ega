const COEF: i16 = 16;
const KERNEL_X: isize = 1;
const KERNEL_WIDTH: isize = 3;
const KERNEL_HEIGHT: isize = 2;
const KERNEL: [[i16; KERNEL_WIDTH as usize]; KERNEL_HEIGHT as usize] = [
    [0, 0, 7],
    [3, 5, 1],
];

pub struct Dither {
    dither_buffer: [[[i16; super::WIDTH + 200] ; 8 ] ; 3 ],
}

impl Dither {
    pub fn new() -> Self {
        Self {
            dither_buffer: [[[0; super::WIDTH + 200] ; 8 ] ; 3 ],
        }
    }

    pub fn dither_get_pixel(&mut self, px: u32, i: isize, j: isize, w: isize) -> usize {
        let mut r: i16 = RED8(px) + self.dither_buffer[0][j as usize % 8][i as usize];
        let mut g: i16 = GREEN8(px) + self.dither_buffer[1][j as usize % 8][i as usize];
        let mut b: i16 = BLUE8(px) + self.dither_buffer[2][j as usize % 8][i as usize];

        self.dither_buffer[0][j as usize % 8][i as usize] = 0;
        self.dither_buffer[1][j as usize % 8][i as usize] = 0;
        self.dither_buffer[2][j as usize % 8][i as usize] = 0;

        r = 0.max(255.min(r));
        g = 0.max(255.min(g));
        b = 0.max(255.min(b));

        let closest = find_closest_palette(r, g, b);

        let rErr: i16 = r - RED8(PALETTE[closest]);
        let gErr: i16 = g - GREEN8(PALETTE[closest]);
        let bErr: i16 = b - BLUE8(PALETTE[closest]);

        for k in 0..(KERNEL_HEIGHT as isize) {
            for l in (-(KERNEL_X as isize))..(KERNEL_WIDTH as isize - KERNEL_X as isize) {
                if !(0 <= i + l && i + l < w) {
                    continue;
                }
                self.dither_buffer[0][((j + k) % 8) as usize][(i + l) as usize] += (KERNEL[k as usize][l as usize + KERNEL_X as usize] * rErr) / COEF;
                self.dither_buffer[1][((j + k) % 8) as usize][(i + l) as usize] += (KERNEL[k as usize][l as usize + KERNEL_X as usize] * gErr) / COEF;
                self.dither_buffer[2][((j + k) % 8) as usize][(i + l) as usize] += (KERNEL[k as usize][l as usize + KERNEL_X as usize] * bErr) / COEF;
            }
        }
        return closest;
    }
}
