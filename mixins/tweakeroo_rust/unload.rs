fn tweakeroo_reset_on_world_unload(_game: &mut crate::game::Game<'_>, _dimension: &str) {
    crate::game::tweakeroo_rust::reset_zoom();
}
