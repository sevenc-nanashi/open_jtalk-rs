use super::*;
use std::mem::MaybeUninit;

#[derive(Default)]
pub struct JpCommon(Option<open_jtalk_sys::JPCommon>);

pub struct JpCommonFeature;

impl resources::Resource for JpCommon {
    fn initialize(&mut self) -> bool {
        if self.0.is_some() {
            panic!("already initialized jpcommon");
        }
        unsafe {
            #[allow(clippy::uninit_assumed_init)]
            let mut jpcommon: open_jtalk_sys::JPCommon = MaybeUninit::uninit().assume_init();
            open_jtalk_sys::JPCommon_initialize(&mut jpcommon);
            self.0 = Some(jpcommon);
        }
        true
    }
    fn clear(&mut self) -> bool {
        unsafe { open_jtalk_sys::JPCommon_clear(self.as_raw_ptr()) };
        self.0 = None;
        true
    }
}

impl JpCommon {
    unsafe fn as_raw_ptr(&self) -> *mut open_jtalk_sys::JPCommon {
        if self.0.is_none() {
            panic!("uninitialized jpcommon");
        }
        self.0.as_ref().unwrap() as *const open_jtalk_sys::JPCommon as *mut open_jtalk_sys::JPCommon
    }

    pub fn refresh(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_refresh(self.as_raw_ptr()) }
    }

    pub fn make_label(&mut self) {
        unsafe { open_jtalk_sys::JPCommon_make_label(self.as_raw_ptr()) }
    }

    pub fn get_label_size(&self) -> i32 {
        unsafe { open_jtalk_sys::JPCommon_get_label_size(self.as_raw_ptr()) }
    }
    pub fn get_label_feature(&self) -> Option<&JpCommonFeature> {
        unsafe {
            let feature = open_jtalk_sys::JPCommon_get_label_feature(self.as_raw_ptr());
            if !feature.is_null() {
                Some(&*(feature as *const JpCommonFeature))
            } else {
                None
            }
        }
    }

    pub fn get_label_feature_mut(&mut self) -> Option<&mut JpCommonFeature> {
        self.get_label_feature().map(|feature| unsafe {
            #[allow(clippy::cast_ref_to_mut)]
            &mut *(feature as *const JpCommonFeature as *mut JpCommonFeature)
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use resources::Resource as _;
    #[rstest]
    fn jpcommon_initialize_and_clear_works() {
        let mut jpcommon = JpCommon::default();
        assert!(jpcommon.initialize());
        assert!(jpcommon.clear());
    }

    #[rstest]
    fn jpcommon_get_label_size_before_make_labelworks() {
        let jpcommon = ManagedResource::<JpCommon>::initialize();
        assert_eq!(0, jpcommon.get_label_size());
    }

    #[rstest]
    fn jpcommon_get_label_feature_mut_before_make_label_works() {
        let mut jpcommon = ManagedResource::<JpCommon>::initialize();

        assert!(jpcommon.get_label_feature_mut().is_none());
    }

    #[rstest]
    fn jpcommon_refresh_works() {
        let mut jpcommon = ManagedResource::<JpCommon>::initialize();
        jpcommon.refresh();
    }
}
