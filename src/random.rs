use rand::Rng;

pub fn random_range(min: usize, max: usize) -> usize {
   return rand::thread_rng().gen_range(min..max);
}
