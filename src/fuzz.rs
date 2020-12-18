use rand::prelude::*;

#[derive(Debug)]
pub struct InputMutator {
    mutation: String,
    rng: StdRng,
}

enum MutateMethod {
    ChangeRandomUTF8,
    InsertRandomUTF8,
    DeleteRandomUTF8,
}

fn generate_random_utf8(rng: &mut StdRng) -> char {
    rng.gen::<char>()
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
        if self.mutation.len() == 0 {
            return;
        }
        let i = self.random_index();
        self.mutation.remove(i);
        self.mutation.insert(i, generate_random_utf8(&mut self.rng));
    }

    fn ins_random_utf8(&mut self) {
        let i = self.random_index();
        self.mutation.insert(i, generate_random_utf8(&mut self.rng));
    }

    fn del_random_utf8(&mut self) {
        if self.mutation.len() == 0 {
            return;
        }
        let i = self.random_index();
        self.mutation.remove(i);
    }

    fn random_index(&mut self) -> usize {
        self.rng.gen_range(0, self.mutation.len())
    }

    pub fn get_mutation(&self) -> &String {
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
        let im = InputMutator::new("hello mutator!\n");
        println!("{:?}", im);
    }

    #[test]
    fn show_change_random_utf8() {
        let mut im = InputMutator::new("hello mutator!\n");
        im.change_random_utf8();
        println!("{:?}", im);
    }

    #[test]
    fn show_ins_random_utf8() {
        let mut im = InputMutator::new("hello mutator!\n");
        im.ins_random_utf8();
        println!("{:?}", im);
    }

    #[test]
    fn show_del_random_utf8() {
        let mut im = InputMutator::new("hello mutator!\n");
        im.del_random_utf8();
        println!("{:?}", im);
    }

    #[test]
    fn show_mutate() {
        let mut im = InputMutator::new("hello mutator!\n");
        im.mutate();
        println!("{:?}", im);
        im.mutate();
        println!("{:?}", im);
        im.mutate();
        println!("{:?}", im);
    }
}
