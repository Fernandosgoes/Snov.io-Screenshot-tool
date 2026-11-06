//! Fonte da marca (Inter) embutida no .exe: é carregada só para este processo, sem instalar
//! nada no Windows. Assim os textos saem em Inter também em outros computadores.

use std::ffi::c_void;
use windows::Win32::Graphics::Gdi::AddFontMemResourceEx;

/// Regular e Bold: os textos desenhados usam o peso mais próximo do 600.
const FONTS: [&[u8]; 2] = [
    include_bytes!("../assets/fonts/Inter-Regular.ttf"),
    include_bytes!("../assets/fonts/Inter-Bold.ttf"),
];

/// Registra as fontes embutidas. Chamar uma vez, antes de criar qualquer fonte.
pub fn load() {
    for data in FONTS {
        let mut n = 0u32;
        unsafe {
            AddFontMemResourceEx(data.as_ptr() as *const c_void, data.len() as u32, None, &mut n);
        }
    }
}
