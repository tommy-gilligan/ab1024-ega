const PALETTE: [u32; 7] = [
    0x00000000,
    0x00FFFFFF,
    0x0000FF00,
    0x000000FF,
    0x00FF0000,
    0x00FFFF00,
    0x00FF8000,
];

const COEF: i16 = 16;
const KERNEL_X: isize = 1;
const KERNEL_WIDTH: isize = 3;
const KERNEL_HEIGHT: isize = 2;
const KERNEL: [[i16; KERNEL_WIDTH as usize]; KERNEL_HEIGHT as usize] = [
    [0, 0, 7],
    [3, 5, 1],
];

#[inline]
pub fn RED8(a: u32) -> i16 {
    (((a) >> 16) & 0xff) as i16
}
#[inline]
pub fn GREEN8(a: u32) -> i16 {
    (((a) >> 8) & 0xff) as i16
}
#[inline]
pub fn BLUE8(a: u32) -> i16 {
    (((a)) & 0xff) as i16
}
#[inline]
fn SQR(a: i16) -> i32 {
    (a as i32) * (a as i32)
}

fn find_closest_palette(r: i16, g: i16, b: i16) -> usize {
    let mut min_distance: i32 = 0x7fffffff;
    let mut contender_count: usize = 0;
    let mut contender_list: [usize; 7] = [0; 7];

    for i in 0..7 {
        let pr: i16 = RED8(PALETTE[i]);
        let pg: i16 = GREEN8(PALETTE[i]);
        let pb: i16 = BLUE8(PALETTE[i]);

        let current_distance: i32 = SQR(r - pr) + SQR(g - pg) + SQR(b - pb);
        if current_distance < min_distance {
            min_distance = current_distance;
            contender_list[0] = i;
            contender_count = 1;
        } else if current_distance == min_distance {
            contender_list[contender_count] = i;
            contender_count += 1;
        }
    }

    // first if single, otherwise random
    return contender_list[0];
}

const WIDTH: usize = 600;

pub struct Dither {
    dither_buffer: [[[i16; WIDTH + 200] ; 8 ] ; 3 ],
}

impl Dither {
    pub fn new() -> Self {
        Self {
            dither_buffer: [[[0; WIDTH + 200] ; 8 ] ; 3 ],
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
