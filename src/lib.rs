use horizon_mod_sdk::*;

pub struct AppleSkinMod;

impl Mod for AppleSkinMod {
    fn init(&mut self) {
        log::info("AppleSkin Horizon aktif!");
    }

    fn frame(&mut self) {
        if let Some(player) = game::get_local_player() {
            let hunger = player.hunger();
            let sat = player.saturation();
            let text = format!("🍗 {} | Sat: {:.1}", hunger, sat);
            hud::draw_text(
                &text,
                hud::Anchor::BottomRight,
                -110,
                -45,
                hud::Color::GOLD,
            );
        }
    }
}

// Makro ini yang otomatis menyambungkan ke 'horizon:mods/events@0.1.0'
export_mod!(AppleSkinMod);
