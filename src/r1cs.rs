use crate::field::FieldElement;

#[derive(Debug, Clone)]
pub struct R1CS {
    pub a: Vec<Vec<FieldElement>>,
    pub b: Vec<Vec<FieldElement>>,
    pub c: Vec<Vec<FieldElement>>,
    pub num_inputs: usize,
    pub num_variables: usize,
}

impl R1CS {
    pub fn new(
        a: Vec<Vec<FieldElement>>,
        b: Vec<Vec<FieldElement>>,
        c: Vec<Vec<FieldElement>>,
        num_inputs: usize,
    ) -> Self {
        let num_variables = a[0].len();
        R1CS { a, b, c, num_inputs, num_variables }
    }

    pub fn is_satisfied(&self, w: &[FieldElement]) -> bool {
        assert_eq!(w.len(), self.num_variables);
        for i in 0..self.a.len() {
            let mut val_a = FieldElement::zero();
            let mut val_b = FieldElement::zero();
            let mut val_c = FieldElement::zero();

            for j in 0..self.num_variables {
                val_a = val_a + self.a[i][j] * w[j];
                val_b = val_b + self.b[i][j] * w[j];
                val_c = val_c + self.c[i][j] * w[j];
            }

            if val_a * val_b != val_c {
                return false;
            }
        }
        true
    }
}
