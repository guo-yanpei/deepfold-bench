pub mod mat_mult;

mod tests {
    use csv::Writer;
    use util::algebra::field::mersenne61_ext::Mersenne61Ext;

    use crate::mat_mult::{mat_mult, naive_mat_mult, Matrix};

    #[test]
    fn test_mat_mult() {
        let mat_a = Matrix::<Mersenne61Ext>::sample(150, 768);
        let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
        let mat_c = mat_a.clone() * mat_b.clone();

        let mut wtr = Writer::from_path("mat_mult.csv").unwrap();
        let size = mat_mult(&mat_a, &mat_b, &mat_c);
        wtr.write_record(&[size.to_string()]).unwrap();
    }

    #[test]
    fn test_naive_mat_mult() {
        let mat_a = Matrix::<Mersenne61Ext>::sample(150, 768);
        let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
        let mat_c = mat_a.clone() * mat_b.clone();

        let mut wtr = Writer::from_path("naive_mat_mult.csv").unwrap();
        let size = naive_mat_mult(&mat_a, &mat_b, &mat_c);
        wtr.write_record(&[size.to_string()]).unwrap();
    }
}
