#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
pub mod test {
    use super::*;

    use root::morfeusz;

    #[test]
    fn test_sanity() {
        unsafe {
            let morf = morfeusz::Morfeusz::createInstance(
                morfeusz::MorfeuszUsage_BOTH_ANALYSE_AND_GENERATE,
            );

	    let m_ref = *morf;

            let res = m_ref.analyse("wlazł kotek na płotek");
        }
    }
}
