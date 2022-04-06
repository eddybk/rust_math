#[allow(dead_code)]
#[derive(Debug)]
/// A 2 dimensional array
pub struct Matrix {
    shape: (i32, i32),
    data: Vec<Vec<f64>>,
}
impl Matrix {
    /// Creates a new matrix with given shape.
    /// The matrix is initiated with zeros.
    pub fn new(rows: i32, cols: i32) -> Matrix {
        Matrix {
            shape: (rows, cols),
            data: vec![vec![0.0; cols as usize]; rows as usize],
        }
    }
    /// returns a tuple with number or rows and number of columns
    pub fn shape(&self) -> (i32, i32) {
        self.shape
    }
    /// clones the data and returns it
    pub fn as_vec(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }
    pub fn ones(rows: i32, cols: i32) -> Matrix {
        Matrix {
            shape: (rows, cols),
            data: vec![vec![1.0; cols as usize]; rows as usize],
        }
    }
    /// Creates a new matrix with given shape.
    /// The matrix is initiated with zeros.
    pub fn random(shape: (i32, i32), range: (f32, f32)) -> Matrix {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut v = vec![];
        for _ in 0..shape.0 {
            let mut temp = vec![];
            for _ in 0..shape.1 {
                temp.push(rng.gen_range(range.0..range.1) as f64);
            }
            v.push(temp);
        }
        Matrix {
            shape: (shape.0, shape.1),
            data: v,
        }
    }
    /// ### matrix.set
    /// Sets self.data to parameter data
    /// ### Panics
    /// > if the parameter datas shape doesnt match self.shape
    /// ## Example
    /// ```
    /// use math::matrix;
    /// let mut m = matrix::Matrix::new(2, 3);
    /// m.set(vec![vec![1., 2., 3.], vec![4., 5., 6.]]);
    /// assert_eq!(m.as_vec(), vec![vec![1., 2., 3.], vec![4., 5., 6.]]);
    /// ```
    pub fn set(&mut self, data: Vec<Vec<f64>>) {
        if data.len() != self.shape.0.try_into().unwrap()
            || data[0].len() != self.shape.1.try_into().unwrap()
        {
            panic!("Can not set matrix to a mishaped vector!")
        }
        self.data = data;
    }
    #[allow(non_snake_case)]
    /// returns trasposed self
    pub fn T(&self) -> Matrix {
        let mut ret = vec![vec![0.0; self.shape.0 as usize]; self.shape.1 as usize];
        for i in 0..self.shape.1 as usize {
            for j in 0..self.shape.0 as usize {
                ret[i][j] = self.data[j][i];
            }
        }
        Matrix {
            shape: (self.shape.1, self.shape.0),
            data: ret,
        }
    }
    /// calculates the dot product of two matrices
    pub fn dot(a: Matrix, b: Matrix) -> Matrix {
        if b.shape.0 != a.shape.1 || a.shape.1 != b.shape.0 {
            panic!(
                "Can not multiply matrices with shapes {:?} and {:?}",
                a.shape, b.shape
            );
        }
        let mut result = Matrix::new(a.shape.0, b.shape.1);
        for i in 0..result.shape.0 {
            for j in 0..result.shape.1 {
                for k in 0..a.shape.1 {
                    result.data[i as usize][j as usize] +=
                        a.data[i as usize][k as usize] * b.data[k as usize][j as usize];
                }
            }
        }
        result
    }
}
impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        let mut v = self.data;
        for i in 0..v.len() {
            for j in 0..v.len() {
                v[i][j] += rhs.data[i][j]
            }
        }
        Matrix {
            shape: self.shape,
            data: v,
        }
    }
}
impl std::ops::AddAssign<Matrix> for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                self.data[i][j] += rhs.data[i][j]
            }
        }
    }
}
impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        let mut v = self.data;
        for i in 0..v.len() {
            for j in 0..v.len() {
                v[i][j] -= rhs.data[i][j]
            }
        }
        Matrix {
            shape: self.shape,
            data: v,
        }
    }
}
impl std::ops::SubAssign<Matrix> for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                self.data[i][j] -= rhs.data[i][j]
            }
        }
    }
}
impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        let mut v = self.data;
        for i in 0..v.len() {
            for j in 0..v.len() {
                v[i][j] += rhs.data[i][j]
            }
        }
        Matrix {
            shape: self.shape,
            data: v,
        }
    }
}
impl std::ops::MulAssign<Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                self.data[i][j] += rhs.data[i][j]
            }
        }
    }
}
impl std::ops::Div<Matrix> for Matrix {
    type Output = Matrix;
    fn div(self, rhs: Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        let mut v = self.data;
        for i in 0..v.len() {
            for j in 0..v.len() {
                v[i][j] += rhs.data[i][j]
            }
        }
        Matrix {
            shape: self.shape,
            data: v,
        }
    }
}
impl std::ops::DivAssign<Matrix> for Matrix {
    fn div_assign(&mut self, rhs: Matrix) {
        if self.shape != rhs.shape {
            panic!("Can not add two matrices of different shapes!");
        }
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                self.data[i][j] += rhs.data[i][j]
            }
        }
    }
}
