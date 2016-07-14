use my_mod::BestTraitEvar;

pub struct BestImplEvar;

impl BestTraitEvar for BestImplEvar {
    fn do_some_of_the_things(&mut self) -> Result<u32, String> {
        let value = Err("I honestly never saw this coming".to_owned());
        debug!("I'm about to return: {:?}", value);
        value
    }
}
