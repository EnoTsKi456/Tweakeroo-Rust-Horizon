fn tweakeroo_reset_on_world_unload(game: &mut crate::game::Game<'_>, dimension: &str) {
    let _ = (game, dimension);
    crate::game::tweakeroo_rust::reset_zoom();
}
