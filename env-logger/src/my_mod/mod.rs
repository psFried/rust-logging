mod best_impl_evar;
mod second_best_impl_evar;

use self::best_impl_evar::BestImplEvar;
use self::second_best_impl_evar::SecondBestImplEvar;

pub trait BestTraitEvar {
    fn do_some_of_the_things(&mut self) -> Result<u32, String>;

    fn do_all_the_things(&mut self) {
        match self.do_some_of_the_things() {
            Ok(value) => info!("Got Value: {}", value),
            Err(message) => error!("Oh no! do_all_the_things returned error: {}", message)
        }
    }
}

pub fn do_stuff() {
    trace!("starting to do stuff");
    let mut best = &mut BestImplEvar as &mut BestTraitEvar;
    best.do_all_the_things();

    let mut second_best = &mut SecondBestImplEvar as &mut BestTraitEvar;
    second_best.do_all_the_things();
    info!("finished doing stuff");
}

