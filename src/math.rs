use collada::mesh::Vertex;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct Matrix4CreationError {
    given_array_size: usize,
}

impl Display for Matrix4CreationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Expected length 16 array, received length {} array", self.given_array_size)
    }
}

impl Error for Matrix4CreationError {}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex for Vector3 {
    fn from_attributes<'a>(attributes: impl Iterator<Item = (&'a str, &'a [String], Option<&'a [f32]>)>) -> Option<Vector3> {
        let mut found_position = false;
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for (name, param_names, data) in attributes {
            if name != "POSITION" {
                continue;
            }
            if found_position {
                return None;
            }
            found_position = true;
            
            let data = data?;
            if data.len() != 3 {
                return None;
            }
            let mut x_ind = None;
            let mut y_ind = None;
            let mut z_ind = None;

            for (i, name) in param_names.iter().enumerate() {
                match name.as_ref() {
                    "X" => x_ind = Some(i),
                    "Y" => y_ind = Some(i),
                    "Z" => z_ind = Some(i),
                    _ => {}
                }
            }

            if x_ind.is_none() || y_ind.is_none() || z_ind.is_none() {
                return None;
            }

            x = data[x_ind.unwrap()];
            y = data[y_ind.unwrap()];
            z = data[z_ind.unwrap()];
        }

        Some(Vector3{ x, y, z })        
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
// Column major
pub struct Matrix4 {
    pub values: [f32; 16],
}

impl Matrix4 {
    pub fn as_ref(&self) -> &[f32] {
        &self.values[..]
    }
    
    pub fn identity() -> Matrix4 {
        let values = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Matrix4 {
            values
        }
    }

    pub fn from_slice(slice: &[f32]) -> Result<Matrix4, Matrix4CreationError> {
        let len = slice.len();
        if len == 16 {
            let mut values = [0.0_f32; 16];
            for (i, value) in slice.iter().enumerate() {
                values[i] = *value;
            }
            Ok(Matrix4 { values })
        }
        else {
            Err(Matrix4CreationError { given_array_size: len })
        }
    }
    
    pub fn transpose(&self) -> Matrix4 {
        let mut matrix = Matrix4::identity();
        
        for i in 0..4 {
            let row = self.get_row(i);
            matrix.set_column(i, row);
        }
        
        matrix
    }

    pub fn print_mat(&self) {
        for i in 0..4 {
            for j in 0..4 {
                let index = j * 4 + i;
                print!("{}  ", self.values[index]);
            }
            println!();
        }
    }
    
    pub fn get_column(&self, index: usize) -> [f32; 4] {
        assert!(index < 4);
        let column = index * 4;
        [
            self.values[column],
            self.values[column + 1],
            self.values[column + 2],
            self.values[column + 3]
        ]
    }
    
    pub fn get_row(&self, index: usize) -> [f32; 4] {
        assert!(index < 4);
        [
            self.values[index],
            self.values[index + 4],
            self.values[index + 8],
            self.values[index + 12]
        ]
    }
    
    pub fn set_column(&mut self, column: usize, value: [f32; 4]) {
        assert!(column < 4);
        let column = column * 4;
        self.values[column] = value[0];
        self.values[column + 1] = value[1];
        self.values[column + 2] = value[2];
        self.values[column + 3] = value[3];
    }

    pub fn set_translation(&mut self, trans: [f32; 3]) {
        self.values[12] = trans[0];
        self.values[13] = trans[1];
        self.values[14] = trans[2];
    }

    pub fn scale(&mut self, scale: [f32; 3]) {
        for i in 0..3 {
            let start = i * 4;
            for j in 0..3 {
                let index = start + j;
                self.values[index] *= scale[i]; 
            }
        }
    }
}
