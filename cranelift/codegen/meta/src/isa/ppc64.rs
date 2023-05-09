use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::{SettingGroup, SettingGroupBuilder};

use crate::shared::Definitions as SharedDefinitions;

fn define_settings(_shared: &SettingGroup) -> SettingGroup {
    let mut settings = SettingGroupBuilder::new("ppc64");

    let _big_endian = settings.add_bool("big_endian", "is big endian", "", false);
    let _has_altivec = settings.add_bool("has_altivec", "supports altivec/vmx", "", false);
    let _has_vsx = settings.add_bool("has_vsx", "supports vsx", "", false);
    settings.build()
}

pub(crate) fn define(shared_defs: &mut SharedDefinitions) -> TargetIsa {
    let settings = define_settings(&shared_defs.settings);
    TargetIsa::new("ppc64", settings)
}
