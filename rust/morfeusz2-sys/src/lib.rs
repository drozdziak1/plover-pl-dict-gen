use autocxx::{prelude::*, subclass::subclass, moveit::MakeCppStorage};

include_cpp! {
    #include "morfeusz2.h"
    safety!(unsafe_ffi)
    generate_ns!("morfeusz")
}


use std::pin::Pin;

pub use ffi::morfeusz;

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
        let mut result_ptr = unsafe {UniquePtr::from_raw(morf_ref.analyse(&input))};

        unsafe {
	    while result_ptr.pin_mut().hasNext() {
		println!("Iteratin' this boiii");
		let next = result_ptr.pin_mut().next().within_unique_ptr();

		println!("isIgn: {}", next.isIgn());
		println!("isWhitespace: {}", next.isWhitespace());
            }

	    let resolver = morf_ref.getIdResolver();

	    let tag_cnt = resolver.getTagsCount();

	    println!("Got {} tags", tag_cnt);
	}
    }
}
