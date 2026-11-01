wit_bindgen::generate!({
    world: "mod-world",
    path: "wit",
});

use exports::horizon::mods::events::{Guest, Registration};

struct AppleSkin;

impl Guest for AppleSkin {
    // Dipanggil sekali saat game membuka mod
    fn init() -> Result<Registration, String> {
        Ok(Registration {
            commands: vec![],
            tick: true,
            frame: true,
            chat: false,
        })
    }

    // Dipanggil setiap detak tick (20 kali per detik di dalam game)
    fn tick(_tick: u64) {
        // Logika pembacaan status pemain
    }

    // Dipanggil setiap frame layar
    fn frame(_seconds: f32) {
        // Logika render teks HUD AppleSkin
    }

    // Menangani perintah chat (jika ada)
    fn command(_name: String, _args: String) -> Result<Option<String>, String> {
        Ok(None)
    }

    // Menangani pesan chat masuk
    fn chat(_line: String) -> Result<Option<String>, String> {
        Ok(None)
    }
}

export!(AppleSkin);
