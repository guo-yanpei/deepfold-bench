use util::{algebra::field::Field, merkle_tree::MERKLE_ROOT_SIZE, query_result::QueryResult};

pub mod prover;
pub mod verifier;

#[derive(Clone)]
pub struct DeepEval<T: Field> {
    point: Vec<T>,
    first_eval: T,
    else_evals: Vec<T>,
}

impl<T: Field> DeepEval<T> {
    pub fn new(point: Vec<T>, poly_hypercube: Vec<T>) -> Self {
        DeepEval {
            point: point.clone(),
            first_eval: Self::evaluatioin_at(point, poly_hypercube),
            else_evals: vec![],
        }
    }

    fn evaluatioin_at(point: Vec<T>, mut poly_hypercube: Vec<T>) -> T {
        let mut len = poly_hypercube.len();
        assert_eq!(len, 1 << point.len());
        for v in point.into_iter() {
            len >>= 1;
            for i in 0..len {
                poly_hypercube[i] *= T::from_int(1) - v;
                let tmp = poly_hypercube[i + len] * v;
                poly_hypercube[i] += tmp;
            }
        }
        poly_hypercube[0]
    }

    pub fn append_else_eval(&mut self, poly_hypercube: Vec<T>) {
        let mut point = self.point[self.else_evals.len()..].to_vec();
        point[0] += T::from_int(1);
        self.else_evals
            .push(Self::evaluatioin_at(point, poly_hypercube));
    }

    pub fn verify(&self, challenges: &Vec<T>) -> T {
        let (_, challenges) = challenges.split_at(challenges.len() - self.point.len());
        let mut y_0 = self.first_eval;
        assert_eq!(self.point.len(), self.else_evals.len());
        for ((x, eval), challenge) in self
            .point
            .iter()
            .zip(self.else_evals.iter())
            .zip(challenges.into_iter())
        {
            let y_1 = eval.clone();
            y_0 += (y_1 - y_0) * (challenge.clone() - x.clone());
        }
        y_0
    }
}

pub struct Commit<T: Field> {
    merkle_root: [u8; MERKLE_ROOT_SIZE],
    deep: T,
}

#[derive(Clone)]
pub struct Proof<T: Field> {
    merkle_root: Vec<[u8; MERKLE_ROOT_SIZE]>,
    query_result: Vec<QueryResult<T>>,
    deep_evals: Vec<(T, Vec<T>)>,
    shuffle_evals: Vec<T>,
    evaluation: T,
    final_value: T,
}

#[cfg(test)]
mod tests {

    use crate::{prover::Prover, verifier::Verifier};
    use util::{
        algebra::{
            coset::Coset,
            field::{mersenne61_ext::Mersenne61Ext, Field},
            polynomial::MultilinearPolynomial,
        },
        random_oracle::RandomOracle,
    };
    use util::{CODE_RATE, SECURITY_BITS};

    fn output_proof_size(variable_num: usize) -> usize {
        let polynomial = MultilinearPolynomial::random_polynomial(variable_num);
        let mut interpolate_cosets = vec![Coset::new(
            1 << (variable_num + CODE_RATE),
            Mersenne61Ext::from_int(1),
        )];
        for i in 1..variable_num {
            interpolate_cosets.push(interpolate_cosets[i - 1].pow(2));
        }
        let oracle = RandomOracle::new(variable_num, SECURITY_BITS / CODE_RATE);
        let prover = Prover::new(variable_num, &interpolate_cosets, polynomial, &oracle);
        let commit = prover.commit_polynomial();
        let verifier = Verifier::new(variable_num, &interpolate_cosets, commit, &oracle);
        let point = verifier.get_open_point();
        let proof = prover.generate_proof(point);
        assert!(verifier.verify(proof));
        0
        // proof.iter().map(|x| x.proof_size()).sum::<usize>()
        //     + variable_num * (MERKLE_ROOT_SIZE + size_of::<Mersenne61Ext>() * 3)
    }

    #[test]
    fn test_proof_size() {
        for i in 10..11 {
            let proof_size = output_proof_size(i);
            println!(
                "Basefold pcs proof size of {} variables is {} bytes",
                i, proof_size
            );
        }
    }
}
