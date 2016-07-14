use my_mod::BestTraitEvar;

pub struct SecondBestImplEvar;

impl BestTraitEvar for SecondBestImplEvar {
    fn do_some_of_the_things(&mut self) -> Result<u32, String> {
        let value = Ok(78);
        debug!("I'm about to return: {:?}", value);
        value
    }
}
