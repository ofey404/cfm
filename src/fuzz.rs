use crate::fuzz::Error::MutationZeroLength;
use rand::prelude::*;

#[derive(Debug)]
pub struct InputMutator {
    mutation: Vec<u8>,
    rng: StdRng,
}

enum Error {
    MutationZeroLength,
}

enum MutateMethod {
    ChangeRandomUTF8,
    InsertRandomUTF8,
    DeleteRandomUTF8,
}

fn generate_random_u8(rng: &mut StdRng) -> u8 {
    rng.gen::<u8>()
}

impl InputMutator {
    pub fn new(seed: &[u8]) -> Self {
        // TODO: A true random mutator.
        let rng_seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ]; // byte array
        InputMutator {
            mutation: seed.to_vec(),
            rng: StdRng::from_seed(rng_seed),
        }
    }

    pub fn mutate(&mut self) {
        // FIXIT: I don't know how to randomly choose a method. I tried:
        // let all_mutate_callbacks = [|| self.write_random_utf8(), || self.bit_flip(),];
        // let random_mutate_method = all_mutate_callbacks.choose(&mut self.rng)?;
        let all_mutate = [
            MutateMethod::ChangeRandomUTF8,
            MutateMethod::InsertRandomUTF8,
            MutateMethod::DeleteRandomUTF8,
        ];
        match all_mutate
            .choose(&mut self.rng)
            .expect("Random choose mutate method failed")
        {
            MutateMethod::ChangeRandomUTF8 => self.change_random_utf8(),
            MutateMethod::InsertRandomUTF8 => self.ins_random_utf8(),
            MutateMethod::DeleteRandomUTF8 => self.del_random_utf8(),
        }
    }

    fn change_random_utf8(&mut self) {
        let i = match self.random_index() {
            Ok(index) => index,
            Err(MutationZeroLength) => return,
        };
        self.mutation.remove(i);
        self.mutation.insert(i, generate_random_u8(&mut self.rng));
    }

    fn ins_random_utf8(&mut self) {
        let i = match self.random_index() {
            Ok(index) => index,
            Err(MutationZeroLength) => 0,
        };
        self.mutation.insert(i, generate_random_u8(&mut self.rng));
    }

    fn del_random_utf8(&mut self) {
        let i = match self.random_index() {
            Ok(index) => index,
            Err(MutationZeroLength) => return,
        };
        self.mutation.remove(i);
    }

    fn bit_flip(&mut self) {
        let i = match self.random_index() {
            Ok(index) => index,
            Err(MutationZeroLength) => return,
        };
        let mut target_byte = self.mutation[i];
        let offset = self.rng.gen_range(0, 8);
        let mask: u8 = 1 << offset;
        self.mutation[i] = (target_byte & !mask) | (!target_byte & mask);
    }

    fn random_index(&mut self) -> Result<usize, Error> {
        if self.mutation.len() == 0 {
            return Err(MutationZeroLength);
        }
        let len = self.mutation.len();
        Ok(self.rng.gen_range(0, self.mutation.len()))
    }

    pub fn get_mutation(&self) -> &Vec<u8> {
        &self.mutation
    }
}

// run with -- --nocapture to check output.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn show_input_mutator_initialization() {
        let seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ];
        let im = InputMutator::new(&seed);
        println!("{:?}", im);
    }

    #[test]
    fn show_change_random_utf8() {
        let seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ];
        let mut im = InputMutator::new(&seed);
        im.change_random_utf8();
        println!("{:?}", im);
    }

    #[test]
    fn show_ins_random_utf8() {
        let seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ];
        let mut im = InputMutator::new(&seed);
        im.ins_random_utf8();
        println!("{:?}", im);
    }

    #[test]
    fn show_del_random_utf8() {
        let seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ];
        let mut im = InputMutator::new(&seed);
        im.del_random_utf8();
        println!("{:?}", im);
    }

    #[test]
    fn show_mutate() {
        let seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ];
        let mut im = InputMutator::new(&seed);
        im.mutate();
        println!("{:?}", im);
        im.mutate();
        println!("{:?}", im);
        im.mutate();
        println!("{:?}", im);
    }

    #[test]
    fn show_generate_u8() {
        let seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ];
        let mut im = InputMutator::new(&seed);
        for _ in 0..100 {
            print!("{}", generate_random_u8(&mut im.rng));
        }
        print!("\n");
    }
}
