pub mod mat_mult;

mod tests {
    use util::algebra::field::mersenne61_ext::Mersenne61Ext;

    use crate::mat_mult::{mat_mult, naive_mat_mult, Matrix};

    // Todo: Naive padding
    #[test]
    fn test_mat_mult() {
        let mat_a = Matrix::<Mersenne61Ext>::sample(150, 768);
        let mat_b = Matrix::<Mersenne61Ext>::sample(768, 2304);
        let mat_c = Matrix::<Mersenne61Ext>::sample(150, 2304);

        mat_mult(&mat_a, &mat_b, &mat_c);
        naive_mat_mult(&mat_a, &mat_b, &mat_c);
    }
}