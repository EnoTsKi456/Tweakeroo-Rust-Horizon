fn tweakeroo_rust_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::tweakeroo_rust as tweakeroo;
    mods.push(crate::game::native_hooks::NativeMod {
        id: tweakeroo::ID.into(),
        title: format!("Tweakeroo Rust {}", tweakeroo::VERSION),
        description: tweakeroo::DESCRIPTION.into(),
        keys: vec![[tweakeroo::KEY.into(), "Zoom key".into(), tweakeroo::DEFAULT_KEY.into()]],
    });
}
