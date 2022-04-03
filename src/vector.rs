#[allow(dead_code)]

pub mod vector {
    #[derive(Debug)]
    pub struct Vector {
        shape: i32,
        data: Vec<f64>
    }
    impl Vector {
        pub fn new(shape: i32) -> Vector {
            Vector {
                shape,
                data: vec![0.0; shape as usize]
            }
        }
        pub fn shape(&self) -> i32 {
            self.shape
        }
        pub fn as_vec(&self) -> Vec<f64> {
            self.data.clone()
        }
    }
}