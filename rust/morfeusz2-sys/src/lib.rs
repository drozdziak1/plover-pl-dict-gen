use autocxx::{prelude::*, subclass::subclass};

include_cpp! {
    #include "morfeusz2.h"
    safety!(unsafe_ffi)
    generate_ns!("morfeusz")
}

pub use ffi::morfeusz;

#[subclass(superclass("morfeusz::MorphInterpretation"))]
pub struct MorphInterpretation {
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sanity() {
        let morf =
            morfeusz::Morfeusz::createInstance(morfeusz::MorfeuszUsage::BOTH_ANALYSE_AND_GENERATE);

        let morf_ref = unsafe {
            morf.as_ref()
                .expect("Could not turn Morfeusz struct into reference")
        };

        cxx::let_cxx_string!(input = "wlazł kotek na płotek");
        let res = morf_ref.analyse(&input);

        while res.hasNext() {
            let morph = res.next();

            morph.getName
        }
    }
}
