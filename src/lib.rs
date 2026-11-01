// Hubungkan dengan definisi world.wit
wit_bindgen::generate!({
    world: "mod-world",
    path: "wit",
});

struct AppleSkin;

// Mengisi pintu gerbang horizon:mods/events@0.1.0
impl exports::horizon::mods::events::Guest for AppleSkin {
    fn init() {
        // Inisialisasi saat game dibuka
    }

    fn frame() {
        // Logika render AppleSkin di layar
    }
}

export!(AppleSkin);
