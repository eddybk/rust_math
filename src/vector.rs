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
        pub fn set(&mut self, values: Vec<f64>) {
            if values.len() != self.shape as usize {
                panic!("Can not set vectors values to {:?} because the length of the vector does not match the shape of the vector!", values)
            }
            self.data = values;
        }
        pub fn dot(&self, other: Vector) -> f64 {
            if other.shape() != self.shape {
                panic!("Can not calculate a dot product of vectors with mismatching shapes!")
            }
            let mut temp = vec![];
            for i in 0..self.data.len() {
                temp.push(self.data[i] * other.data[i]);
            }
            temp.iter().sum::<f64>()
        }
    }
    
    // OPS OVERLOADS

    impl std::ops::Add<f64> for Vector {
        type Output = Vector;
        fn add(self, rhs: f64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] += rhs;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::AddAssign<f64> for Vector {
        fn add_assign(&mut self, rhs: f64) {
            for i in 0..self.data.len() {
                self.data[i] += rhs;
            }
        }
    }
    impl std::ops::Add<f32> for Vector {
        type Output = Vector;
        fn add(self, rhs: f32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] += rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::AddAssign<f32> for Vector {
        fn add_assign(&mut self, rhs: f32) {
            for i in 0..self.data.len() {
                self.data[i] += rhs as f64;
            }
        }
    }
    impl std::ops::Add<i64> for Vector {
        type Output = Vector;
        fn add(self, rhs: i64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] += rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::AddAssign<i64> for Vector {
        fn add_assign(&mut self, rhs: i64) {
            for i in 0..self.data.len() {
                self.data[i] += rhs as f64;
            }
        }
    }
    impl std::ops::Add<i32> for Vector {
        type Output = Vector;
        fn add(self, rhs: i32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] += rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::AddAssign<i32> for Vector {
        fn add_assign(&mut self, rhs: i32) {
            for i in 0..self.data.len() {
                self.data[i] += rhs as f64;
            }
        }
    }
    impl std::ops::Sub<f64> for Vector {
        type Output = Vector;
        fn sub(self, rhs: f64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] -= rhs;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::SubAssign<f64> for Vector {
        fn sub_assign(&mut self, rhs: f64) {
            for i in 0..self.data.len() {
                self.data[i] -= rhs as f64;
            }
        }
    }
    impl std::ops::Sub<f32> for Vector {
        type Output = Vector;
        fn sub(self, rhs: f32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] -= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::SubAssign<f32> for Vector {
        fn sub_assign(&mut self, rhs: f32) {
            for i in 0..self.data.len() {
                self.data[i] -= rhs as f64;
            }
        }
    }
    impl std::ops::Sub<i64> for Vector {
        type Output = Vector;
        fn sub(self, rhs: i64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] -= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::SubAssign<i64> for Vector {
        fn sub_assign(&mut self, rhs: i64) {
            for i in 0..self.data.len() {
                self.data[i] -= rhs as f64;
            }
        }
    }
    impl std::ops::Sub<i32> for Vector {
        type Output = Vector;
        fn sub(self, rhs: i32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] -= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::SubAssign<i32> for Vector {
        fn sub_assign(&mut self, rhs: i32) {
            for i in 0..self.data.len() {
                self.data[i] -= rhs as f64;
            }
        }
    }
    impl std::ops::Mul<f64> for Vector {
        type Output = Vector;
        fn mul(self, rhs: f64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] *= rhs;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::MulAssign<f64> for Vector {
        fn mul_assign(&mut self, rhs: f64) {
            for i in 0..self.data.len() {
                self.data[i] *= rhs as f64;
            }
        }
    }
    impl std::ops::Mul<f32> for Vector {
        type Output = Vector;
        fn mul(self, rhs: f32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] *= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::MulAssign<f32> for Vector {
        fn mul_assign(&mut self, rhs: f32) {
            for i in 0..self.data.len() {
                self.data[i] *= rhs as f64;
            }
        }
    }
    impl std::ops::Mul<i64> for Vector {
        type Output = Vector;
        fn mul(self, rhs: i64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] *= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::MulAssign<i64> for Vector {
        fn mul_assign(&mut self, rhs: i64) {
            for i in 0..self.data.len() {
                self.data[i] *= rhs as f64;
            }
        }
    }
    impl std::ops::Mul<i32> for Vector {
        type Output = Vector;
        fn mul(self, rhs: i32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] *= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::MulAssign<i32> for Vector {
        fn mul_assign(&mut self, rhs: i32) {
            for i in 0..self.data.len() {
                self.data[i] *= rhs as f64;
            }
        }
    }
    impl std::ops::Div<f64> for Vector {
        type Output = Vector;
        fn div(self, rhs: f64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] /= rhs;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::DivAssign<f64> for Vector {
        fn div_assign(&mut self, rhs: f64) {
            for i in 0..self.data.len() {
                self.data[i] /= rhs as f64;
            }
        }
    }
    impl std::ops::Div<f32> for Vector {
        type Output = Vector;
        fn div(self, rhs: f32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] /= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::DivAssign<f32> for Vector {
        fn div_assign(&mut self, rhs: f32) {
            for i in 0..self.data.len() {
                self.data[i] /= rhs as f64;
            }
        }
    }
    impl std::ops::Div<i64> for Vector {
        type Output = Vector;
        fn div(self, rhs: i64) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] /= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::DivAssign<i64> for Vector {
        fn div_assign(&mut self, rhs: i64) {
            for i in 0..self.data.len() {
                self.data[i] /= rhs as f64;
            }
        }
    }
    impl std::ops::Div<i32> for Vector {
        type Output = Vector;
        fn div(self, rhs: i32) -> Self::Output {
            let mut d = self.as_vec();
            for i in 0..d.len() {
                d[i] /= rhs as f64;
            }
            Self {
                shape: self.shape,
                data: d
            }
        }
    }
    impl std::ops::DivAssign<i32> for Vector {
        fn div_assign(&mut self, rhs: i32) {
            for i in 0..self.data.len() {
                self.data[i] /= rhs as f64;
            }
        }
    }
    impl std::ops::Add<Vector> for Vector {
        type Output = Vector;
        fn add(self, rhs: Vector) -> Self::Output {
            if rhs.shape() != self.shape() {
                panic!("Can not add two vectors of mismatching shapes!")
            }
            let mut temp = self.as_vec();
            for i in 0..temp.len() {
                temp[i] += rhs.data[i]
            }
            Vector {
                shape: self.shape,
                data: temp
            }
        }
    }
    impl std::ops::AddAssign<Vector> for Vector {
        fn add_assign(&mut self, rhs: Vector) {
            if rhs.shape() != self.shape() {
                panic!("Can not add two vectors of mismatching shapes!")
            }
            for i in 0..self.data.len() {
                self.data[i] += rhs.data[i]
            }
        }
    }
    impl std::ops::Sub<Vector> for Vector {
        type Output = Vector;
        fn sub(self, rhs: Vector) -> Self::Output {
            if rhs.shape() != self.shape() {
                panic!("Can not subtract two vectors of mismatching shapes!")
            }
            let mut temp = self.as_vec();
            for i in 0..temp.len() {
                temp[i] -= rhs.data[i]
            }
            Vector {
                shape: self.shape,
                data: temp
            }
        }
    }
    impl std::ops::SubAssign<Vector> for Vector {
        fn sub_assign(&mut self, rhs: Vector) {
            if rhs.shape() != self.shape() {
                panic!("Can not subtract two vectors of mismatching shapes!")
            }
            for i in 0..self.data.len() {
                self.data[i] -= rhs.data[i]
            }
        }
    }
    impl std::ops::Mul<Vector> for Vector {
        type Output = Vector;
        fn mul(self, rhs: Vector) -> Self::Output {
            if rhs.shape() != self.shape() {
                panic!("Can not multiply two vectors of mismatching shapes!")
            }
            let mut temp = self.as_vec();
            for i in 0..temp.len() {
                temp[i] *= rhs.data[i]
            }
            Vector {
                shape: self.shape,
                data: temp
            }
        }
    }
    impl std::ops::MulAssign<Vector> for Vector {
        fn mul_assign(&mut self, rhs: Vector) {
            if rhs.shape() != self.shape() {
                panic!("Can not multiply two vectors of mismatching shapes!")
            }
            for i in 0..self.data.len() {
                self.data[i] *= rhs.data[i]
            }
        }
    }
    impl std::ops::Div<Vector> for Vector {
        type Output = Vector;
        fn div(self, rhs: Vector) -> Self::Output {
            if rhs.shape() != self.shape() {
                panic!("Can not divide two vectors of mismatching shapes!")
            }
            let mut temp = self.as_vec();
            for i in 0..temp.len() {
                temp[i] /= rhs.data[i]
            }
            Vector {
                shape: self.shape,
                data: temp
            }
        }
    }
    impl std::ops::DivAssign<Vector> for Vector {
        fn div_assign(&mut self, rhs: Vector) {
            if rhs.shape() != self.shape() {
                panic!("Can not divide two vectors of mismatching shapes!")
            }
            for i in 0..self.data.len() {
                self.data[i] /= rhs.data[i]
            }
        }
    }
}