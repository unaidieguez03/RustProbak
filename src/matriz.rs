pub struct Matriz {
    ancho: u32,
    alto: u32,
    matrix_vec: Vec<f32>,
}

impl Matriz {
    pub fn from_vec(ancho: u32, alto: u32, valores: Vec<f32>) -> Self {
        assert_eq!((ancho * alto) as usize, valores.len(), "Cantidad incorrecta de valores");
        Matriz {
            ancho,
            alto,
            matrix_vec: valores,
        }
    }

    pub fn obtener(&self, fila: u32, columna: u32) -> Option<f32> {
        if fila < self.alto && columna < self.ancho {
            let idx = (fila * self.ancho + columna) as usize;
            self.matrix_vec.get(idx).cloned() // Devuelve Option<f32>
        } else {
            None
        }
    }

    pub fn asignar(&mut self, fila: u32, columna: u32, valor: f32) {
        if fila < self.alto && columna < self.ancho {
            let idx = (fila * self.ancho + columna) as usize;
            self.matrix_vec[idx] = valor;
        }
    }
}
