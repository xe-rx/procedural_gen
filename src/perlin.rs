use rand::Rng;

#[derive(Debug)]
pub struct Perlin {
    pub dim: (i32, i32),
    pub grid: Vec<f32>,
}

impl Perlin {
    pub fn new(size: i32) -> Perlin {
        if size < 0 {
            panic!("size dimensions must be positive");
        }
        // Note the constraint that the perlin grid is always square
        let row: i32 = size;
        let col: i32 = size;
        let mut grid: Vec<f32> = Vec::with_capacity((row * col) as usize);

        // populate each point in grid with random degree-value
        for _ in 0..(row*col) {
            grid.push(rand_degree());
        }
        println!("GRID: {:#?}", grid);

        return Perlin{
            dim: (row, col),
            grid
        };
    }

    pub fn noise_value(&self, x: f32, y: f32) -> f32 {
        if x < 0.0 || y < 0.0 || x > self.dim.0 as f32 || y > self.dim.1 as f32 {
            panic!("coordinates must be within grid")
        }

        // 2d coords of the points around coord (x,y) calculated and converted to 1d index values
        // (degrees)
        let top_left_grid: (i32, i32) = (x.floor() as i32, y.ceil() as i32);
        let top_right_grid: (i32, i32) = (x.ceil() as i32, y.ceil() as i32);
        let bot_left_grid: (i32, i32) = (x.floor() as i32, y.floor() as i32);
        let bot_right_grid: (i32, i32) = (x.ceil() as i32, y.floor() as i32);

        let top_left_deg: f32 = self.index_grid(top_left_grid);
        let top_right_deg: f32 = self.index_grid(top_right_grid);
        let bot_left_deg: f32 = self.index_grid(bot_left_grid);
        let bot_right_deg: f32 = self.index_grid(bot_right_grid);

        // vector theta filled with (degree (in radians!), (x, y)) for each of the four grid points
        let theta: Vec<(f32, (i32, i32))> = Vec::from([ (top_left_deg.to_radians(), top_left_grid),
                                                        (top_right_deg.to_radians(), top_right_grid),
                                                        (bot_left_deg.to_radians(), bot_left_grid),
                                                        (bot_right_deg.to_radians(), bot_right_grid)
        ]);
        let point: (f32, f32) = (x, y);

        // each dot product with corresponding grid point
        let dotproducts: Vec<(f32, (i32, i32))> = Vec::from([   (dot_product(theta[0], point), top_left_grid),
                                                                (dot_product(theta[1], point), top_right_grid),
                                                                (dot_product(theta[2], point), bot_left_grid),
                                                                (dot_product(theta[3], point), bot_right_grid)
        ]);

        return 0.0;
    }

    fn index_grid(&self, idx: (i32, i32)) -> f32 {
        // converts indexing from 2-dimensional to 1d
        let row: usize = idx.0.try_into().expect("Negative or invalid row index");
        let col: usize = idx.1.try_into().expect("Negative or invalid column index");

        let index = row * (self.dim.0 as usize) + col;
        return self.grid[index];
    }
}

// Returns i32 in range of 1 - 360.
fn rand_degree() -> f32 {
    return rand::thread_rng().gen_range(1.0..=360.0);
}

// Calculate dot product between angle and magnitude of vector
fn dot_product(grid_point: (f32, (i32, i32)), point: (f32, f32)) -> f32 {
    let a_x = point.0 - grid_point.1.0 as f32;
    let a_y = point.1 - grid_point.1.1 as f32;

    let gradiant_x = grid_point.0.cos();
    let gradiant_y = grid_point.0.sin();

    return (a_x * gradiant_x) + (a_y * gradiant_y);
}
