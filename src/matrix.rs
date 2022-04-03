#[allow(unused)]

pub mod matrix {
    
    pub struct Matrix {
        shape: (i32, i32),
        data: Vec<Vec<f64>>
    }
    impl Matrix {
        pub fn new(rows: i32, cols: i32) -> Matrix {
            Matrix {
                shape: (rows, cols),
                data: vec![vec![0.0; cols as usize]; rows as usize]
            }
        }
        pub fn shape(&self) -> (i32, i32) {
            self.shape
        }
        pub fn as_vec(&self) -> Vec<Vec<f64>> {
            self.data.clone()
        }
    }
}