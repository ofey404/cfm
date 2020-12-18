use rand::prelude::*;

pub struct InputMutator {
    mutation: String,
    rng: StdRng,
}

enum MutateMethod {
    WriteRandomUTF8,
    BitFlip,
}

impl InputMutator {
    pub fn new(seed: &str) -> Self {
        // TODO: A true random mutator.
        let rng_seed: [u8; 32] = [
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        ]; // byte array
        InputMutator {
            mutation: seed.to_string(),
            rng: StdRng::from_seed(rng_seed),
        }
    }

    pub fn mutate(&mut self) {
        // FIXIT: I don't know how to randomly choose a method. I tried:
        // let all_mutate_callbacks = [|| self.write_random_utf8(), || self.bit_flip(),];
        // let random_mutate_method = all_mutate_callbacks.choose(&mut self.rng)?;
        let all_mutate = [MutateMethod::WriteRandomUTF8, MutateMethod::BitFlip];
        match all_mutate
            .choose(&mut self.rng)
            .expect("Random choose mutate method failed")
        {
            MutateMethod::WriteRandomUTF8 => self.write_random_utf8(),
            MutateMethod::BitFlip => self.bit_flip(),
        }
    }

    fn write_random_utf8(&mut self) {
        // TODO: implement mutate method
        println!("write_random_utf8");
    }

    fn bit_flip(&mut self) {
        // TODO: implement mutate method
        println!("bit_flip");
    }

    pub fn get_mutation(&self) -> &String {
        &self.mutation
    }
}
