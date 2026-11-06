//! Janela de configurações (aberta pelo menu da bandeja), desenhada à mão no estilo da marca
//! Snov.io: card branco, seletores segmentados, bolinhas de cor, barras de espessura e tamanho
//! com prévia, toggle e botões. Só o campo de captura de tecla personalizada é um controle
//! nativo (o Windows precisa dele para ler a combinação). Salvar grava settings.json, registra o
//! novo atalho na hora e ajusta a inicialização automática. Enter salva, Esc fecha.

use crate::config::{Color, Hotkey, HotkeyChoice, ImageFormat, Settings, UiStyle};
use crate::gfx::{Canvas, RectF, argb, argb_from_colorref};
use crate::{hotkey, startup};
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU32, AtomicUsize, Ordering};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontW, CreateRoundRectRgn, CreateSolidBrush, DRAW_TEXT_FORMAT, DT_CENTER, DT_LEFT, DT_NOPREFIX, FillRect,
    DT_RIGHT, DT_SINGLELINE, DT_VCENTER, DeleteObject, DrawTextW, EndPaint, FONT_CHARSET,
    FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HDC, HGDIOBJ, InvalidateRect,
    PAINTSTRUCT, ScreenToClient, SelectObject, SetBkMode, SetTextColor, SetWindowRgn, TRANSPARENT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::{ICC_HOTKEY_CLASS, INITCOMMONCONTROLSEX, InitCommonControlsEx};
use windows::Win32::UI::HiDpi::GetDpiForSystem;
use windows::Win32::UI::Input::KeyboardAndMouse::{EnableWindow, ReleaseCapture, SetCapture};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, FindWindowW, GetDlgItem, HMENU, IDC_ARROW,
    LoadCursorW, MB_ICONINFORMATION, MB_OK, MessageBoxW, RegisterClassW, SetForegroundWindow,
    ShowWindow, SW_HIDE, SW_SHOW, SYSTEM_PARAMETERS_INFO_ACTION,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SendMessageW, SystemParametersInfoW, WINDOW_EX_STYLE,
    WINDOW_STYLE, WM_CLOSE, WM_DESTROY, WM_PAINT, WM_SETFONT, WNDCLASSW, WS_CHILD, WS_POPUP,
    WS_TABSTOP, WS_VISIBLE,
};
use windows::core::{PCWSTR, w};

const CLASS: PCWSTR = w!("ScreenshotterSettings");

/// Controle nativo de captura de tecla (só visível quando "Custom" está escolhido).
const ID_CUSTOM: i32 = 107;
// Campo de captura de tecla (classe nativa do Windows, msctls_hotkey32).
const WM_SETHOTKEY: u32 = 0x0032;
const WM_GETHOTKEY: u32 = 0x0033;
// Bits de modificador do campo de tecla (HOTKEYF_*), diferentes dos MOD_* do RegisterHotKey.
const HKF_SHIFT: usize = 0x01;
const HKF_CONTROL: usize = 0x02;
const HKF_ALT: usize = 0x04;

const WM_ERASEBKGND: u32 = 0x0014;
const WM_NCHITTEST: u32 = 0x0084;
const WM_KEYDOWN: u32 = 0x0100;
const WM_MOUSEMOVE: u32 = 0x0200;
const WM_LBUTTONDOWN: u32 = 0x0201;
const WM_LBUTTONUP: u32 = 0x0202;
const VK_RETURN_KEY: usize = 0x0D;
const VK_ESCAPE_KEY: usize = 0x1B;
const HTCAPTION: isize = 2;

// Tamanho lógico da janela (escalado pelo DPI na hora de abrir).
const W: i32 = 440;
const H: i32 = 528;
/// Faixa do topo onde se arrasta a janela (já que não há barra de título).
const HEADER_H: i32 = 56;

// Cores da marca (0xRRGGBB).
const BG: u32 = 0xF6F5FB;
const WHITE: u32 = 0xFFFFFF;
const BORDER: u32 = 0xB9B2D8;
const CHIP_BORDER: u32 = 0xD6D0EA;
const INK: u32 = 0x2B2555;
const MUTED: u32 = 0x6B6790;
const ACCENT: u32 = 0x6F55E8;
const CHIP_OFF: u32 = 0xF1EFF8;
const CHIP_OFF_INK: u32 = 0x4A4670;
const TRACK: u32 = 0xE4E1F0;
const PLACEHOLDER: u32 = 0xA29DC0;
const FIELD_BG: u32 = 0xFAF9FD;

// Geometria em pixels lógicos: (x, y, largura, altura).
const CARD: (i32, i32, i32, i32) = (16, 64, 408, 400);
const HK_CHIPS: [(i32, i32, i32, i32); 4] = [(32, 98, 86, 30), (126, 98, 86, 30), (220, 98, 86, 30), (314, 98, 86, 30)];
const CUSTOM_FIELD: (i32, i32, i32, i32) = (32, 136, 368, 24);
const SWATCH_Y: i32 = 197;
const SWATCH: i32 = 20;
const SWATCH_STEP: i32 = 58;
/// Barras: x inicial e final e centro y de cada uma (espessura, tamanho do texto).
const TRACK_X0: f32 = 32.0;
const TRACK_X1: f32 = 236.0;
const TRACK_Y: [f32; 2] = [252.0, 294.0];
const VALUE_TEXT: [(i32, i32, i32, i32); 2] = [(246, 240, 60, 24), (246, 282, 60, 24)];
/// Caixas de prévia (espessura e texto).
const PREVIEW: [(i32, i32, i32, i32); 2] = [(300, 234, 108, 36), (300, 276, 108, 36)];
const FMT_CHIPS: [(i32, i32, i32, i32); 2] = [(32, 338, 178, 30), (222, 338, 178, 30)];
const STYLE_CHIPS: [(i32, i32, i32, i32); 2] = [(32, 396, 178, 30), (222, 396, 178, 30)];
const TOGGLE: (i32, i32, i32, i32) = (32, 436, 40, 22);
const TOGGLE_LABEL: (i32, i32, i32, i32) = (84, 432, 300, 30);
const CANCEL_BTN: (i32, i32, i32, i32) = (236, 482, 88, 34);
const SAVE_BTN: (i32, i32, i32, i32) = (332, 482, 76, 34);

// Faixas das barras (espessura do traço e tamanho do texto).
const STROKE_RANGE: (u32, u32) = (1, 20);
const TEXT_RANGE: (u32, u32) = (10, 36);
const RANGES: [(u32, u32); 2] = [STROKE_RANGE, TEXT_RANGE];

/// Valores atuais das barras: espessura e tamanho do texto.
static VALS: [AtomicU32; 2] = [AtomicU32::new(3), AtomicU32::new(18)];
/// Barra sendo arrastada (0, 1) ou 9 se nenhuma.
static DRAG: AtomicU32 = AtomicU32::new(9);
/// Escolhas atuais, como índices nas tabelas abaixo.
static HK: AtomicUsize = AtomicUsize::new(0);
static COLOR: AtomicUsize = AtomicUsize::new(3);
static FMT: AtomicUsize = AtomicUsize::new(0);
static STYLE: AtomicUsize = AtomicUsize::new(0);
static STARTUP: AtomicBool = AtomicBool::new(false);
/// Janela principal (dona do atalho global): precisa ser reregistrada ao salvar.
static MAIN: AtomicIsize = AtomicIsize::new(0);
/// Escala de DPI (bits de f32), para desenhar em pixels reais.
static SCALE: AtomicU32 = AtomicU32::new(0x3F80_0000);

/// Atalhos oferecidos. "Custom" é o último e libera o campo de tecla.
const HOTKEYS: [(HotkeyChoice, &str); 4] = [
    (HotkeyChoice::ShiftBackspace, "Shift+BKSP"),
    (HotkeyChoice::ShiftInsert, "Shift+Insert"),
    (HotkeyChoice::ShiftPrtScn, "Shift+PrtScn"),
    (HotkeyChoice::Custom, "Custom"),
];
const CUSTOM_INDEX: usize = 3;

const FORMATS: [(ImageFormat, &str); 2] = [(ImageFormat::Jpg, "JPEG"), (ImageFormat::Png, "PNG")];

const STYLES: [(UiStyle, &str); 2] = [
    (UiStyle::Conservadora, "Conservative (light)"),
    (UiStyle::Moderna, "Modern (dark)"),
];

const COLORS: [(Color, &str); 7] = [
    (Color::Yellow, "Yellow"),
    (Color::Blue, "Blue"),
    (Color::Green, "Green"),
    (Color::Red, "Red"),
    (Color::Black, "Black"),
    (Color::White, "White"),
    (Color::Purple, "Purple"),
];

/// Formato do campo de tecla: byte baixo = tecla virtual, byte alto = modificadores HOTKEYF_*.
fn to_control(h: Hotkey) -> usize {
    let mut f = 0;
    if h.mods & 4 != 0 {
        f |= HKF_SHIFT;
    }
    if h.mods & 2 != 0 {
        f |= HKF_CONTROL;
    }
    if h.mods & 1 != 0 {
        f |= HKF_ALT;
    }
    (h.vk as usize & 0xFF) | (f << 8)
}

fn from_control(v: usize) -> Hotkey {
    let f = (v >> 8) & 0xFF;
    let mut mods = 0;
    if f & HKF_SHIFT != 0 {
        mods |= 4;
    }
    if f & HKF_CONTROL != 0 {
        mods |= 2;
    }
    if f & HKF_ALT != 0 {
        mods |= 1;
    }
    Hotkey { mods, vk: (v & 0xFF) as u32 }
}

/// Aceita só combinações que o Windows aceita como atalho global e que não viram
/// digitação normal: precisa de Ctrl, Alt ou Shift (ou ser uma tecla F), e não pode ser
/// só um modificador.
fn valid_custom(h: Hotkey) -> Result<(), &'static str> {
    let modifier_only = matches!(h.vk, 0x10..=0x12 | 0x5B | 0x5C | 0xA0..=0xA5);
    if h.vk == 0 {
        return Err("Pick a key in the \"Custom key\" field.");
    }
    if modifier_only {
        return Err("Pick a key other than Ctrl, Alt or Shift alone.");
    }
    let f_key = (0x70..=0x87).contains(&h.vk);
    if h.mods == 0 && !f_key {
        return Err("Use Ctrl, Alt or Shift with the key (or an F key).");
    }
    Ok(())
}

fn scale() -> f32 {
    f32::from_bits(SCALE.load(Ordering::Relaxed))
}

fn send(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe { SendMessageW(h, msg, Some(wp), Some(lp)) }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Retângulo lógico escalado para pixels, em ponto flutuante (para o GDI+).
fn sr(r: (i32, i32, i32, i32), s: f32) -> RectF {
    (r.0 as f32 * s, r.1 as f32 * s, r.2 as f32 * s, r.3 as f32 * s)
}

/// Retângulo lógico escalado para pixels, em inteiros (para o GDI).
fn rect_px(r: (i32, i32, i32, i32), s: f32) -> RECT {
    let sc = |v: i32| (v as f32 * s).round() as i32;
    RECT { left: sc(r.0), top: sc(r.1), right: sc(r.0 + r.2), bottom: sc(r.1 + r.3) }
}

/// Cor 0xRRGGBB convertida para COLORREF (0x00BBGGRR).
fn cref(rgb: u32) -> COLORREF {
    let (r, g, b) = ((rgb >> 16) & 0xFF, (rgb >> 8) & 0xFF, rgb & 0xFF);
    COLORREF(r | (g << 8) | (b << 16))
}

fn inside(p: (f32, f32), r: (i32, i32, i32, i32)) -> bool {
    p.0 >= r.0 as f32 && p.0 < (r.0 + r.2) as f32 && p.1 >= r.1 as f32 && p.1 < (r.1 + r.3) as f32
}

fn first_hit(p: (f32, f32), rects: &[(i32, i32, i32, i32)]) -> Option<usize> {
    rects.iter().position(|r| inside(p, *r))
}

/// Mostra o campo de tecla só quando "Custom" está escolhido.
fn sync_custom(hwnd: HWND) {
    let show = HK.load(Ordering::Relaxed) == CUSTOM_INDEX;
    unsafe {
        let custom = GetDlgItem(Some(hwnd), ID_CUSTOM).unwrap_or_default();
        let _ = ShowWindow(custom, if show { SW_SHOW } else { SW_HIDE });
        let _ = EnableWindow(custom, show);
    }
}

fn redraw(hwnd: HWND) {
    unsafe {
        let _ = InvalidateRect(Some(hwnd), None, false);
    }
}

fn mouse_xy(lp: LPARAM) -> (f32, f32) {
    let s = scale();
    let x = (lp.0 & 0xFFFF) as i16 as f32;
    let y = ((lp.0 >> 16) & 0xFFFF) as i16 as f32;
    (x / s, y / s)
}

/// Valor da barra para a posição x (limitado à faixa).
fn value_at(x: f32, (lo, hi): (u32, u32)) -> u32 {
    let f = ((x - TRACK_X0) / (TRACK_X1 - TRACK_X0)).clamp(0.0, 1.0);
    lo + (f * (hi - lo) as f32).round() as u32
}

fn set_from_x(hwnd: HWND, i: usize, x: f32) {
    let v = value_at(x, RANGES[i]);
    if VALS[i].swap(v, Ordering::Relaxed) != v {
        redraw(hwnd);
    }
}

fn slider(i: usize) -> u32 {
    VALS[i].load(Ordering::Relaxed).clamp(RANGES[i].0, RANGES[i].1)
}

/// Desenha um texto (GDI) dentro do retângulo lógico, com a fonte da marca.
unsafe fn text(hdc: HDC, r: (i32, i32, i32, i32), s: f32, t: &str, px: i32, bold: bool, color: COLORREF, align: DRAW_TEXT_FORMAT) {
    unsafe {
        let font = CreateFontW(
            -((px as f32) * s).round() as i32,
            0,
            0,
            0,
            if bold { 700 } else { 500 },
            0,
            0,
            0,
            FONT_CHARSET(1),
            FONT_OUTPUT_PRECISION(0),
            FONT_CLIP_PRECISION(0),
            FONT_QUALITY(5),
            0,
            w!("Segoe UI"),
        );
        let old = SelectObject(hdc, HGDIOBJ(font.0));
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, color);
        let mut rc = rect_px(r, s);
        let mut wide_text: Vec<u16> = t.encode_utf16().collect();
        DrawTextW(hdc, &mut wide_text, &mut rc, align | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX);
        SelectObject(hdc, old);
        let _ = DeleteObject(HGDIOBJ(font.0));
    }
}

/// Pinta a janela inteira: formas (GDI+) e depois os textos (GDI).
fn paint(hwnd: HWND) {
    let s = scale();
    let hk = HK.load(Ordering::Relaxed);
    let col = COLOR.load(Ordering::Relaxed).min(COLORS.len() - 1);
    let color = COLORS[col].0;
    let fmt = FMT.load(Ordering::Relaxed).min(FORMATS.len() - 1);
    let style = STYLE.load(Ordering::Relaxed).min(STYLES.len() - 1);
    let on = STARTUP.load(Ordering::Relaxed);

    unsafe {
        let mut ps = PAINTSTRUCT::default();
        let hdc = BeginPaint(hwnd, &mut ps);

        // Fundo lilás do tamanho da janela (com GDI, pois raio zero não pinta no GDI+).
        let full = RECT { left: 0, top: 0, right: (W as f32 * s).round() as i32, bottom: (H as f32 * s).round() as i32 };
        let bg_brush = CreateSolidBrush(cref(BG));
        FillRect(hdc, &full, bg_brush);
        let _ = DeleteObject(HGDIOBJ(bg_brush.0));

        if let Some(cv) = Canvas::from_hdc(hdc) {
            // Borda da janela e card branco.
            cv.stroke_round(sr((0, 0, W, H), s), 8.0 * s, argb(0xFF, BORDER), s.max(1.0));
            cv.fill_round(sr(CARD, s), 4.0 * s, argb(0xFF, WHITE));
            cv.stroke_round(sr(CARD, s), 4.0 * s, argb(0xFF, BORDER), s.max(1.0));

            // Chips de atalho.
            for (i, r) in HK_CHIPS.iter().enumerate() {
                chip(&cv, *r, s, i == hk);
            }
            if hk != CUSTOM_INDEX {
                cv.fill_round(sr(CUSTOM_FIELD, s), 3.0 * s, argb(0xFF, FIELD_BG));
                cv.stroke_round(sr(CUSTOM_FIELD, s), 3.0 * s, argb(0xFF, CHIP_BORDER), s.max(1.0));
            }

            // Bolinhas de cor; a escolhida tem anel roxo.
            for (i, (c, _)) in COLORS.iter().enumerate() {
                let r = (32 + i as i32 * SWATCH_STEP, SWATCH_Y, SWATCH, SWATCH);
                let e = sr(r, s);
                cv.fill_ellipse(e, argb_from_colorref(c.colorref()));
                if *c == Color::White {
                    cv.stroke_ellipse(e, argb(0xFF, BORDER), 2.0 * s);
                } else {
                    cv.stroke_ellipse(e, argb(0xFF, CHIP_BORDER), s.max(1.0));
                }
                if i == col {
                    let ring = (e.0 - 4.0 * s, e.1 - 4.0 * s, e.2 + 8.0 * s, e.3 + 8.0 * s);
                    cv.stroke_ellipse(ring, argb(0xFF, ACCENT), 2.0 * s);
                }
            }

            // Barras de espessura e tamanho do texto, com a bolinha de posição.
            for i in 0..2 {
                let (lo, hi) = RANGES[i];
                let f = (slider(i) - lo) as f32 / (hi - lo) as f32;
                let cy = TRACK_Y[i];
                let len = TRACK_X1 - TRACK_X0;
                cv.fill_round(sr((TRACK_X0 as i32, cy as i32 - 2, len as i32, 4), s), 2.0 * s, argb(0xFF, TRACK));
                if f > 0.0 {
                    cv.fill_round(sr((TRACK_X0 as i32, cy as i32 - 2, (len * f) as i32, 4), s), 2.0 * s, argb(0xFF, ACCENT));
                }
                let kx = TRACK_X0 + len * f;
                cv.fill_ellipse(sr(((kx - 8.0) as i32, (cy - 8.0) as i32, 16, 16), s), argb(0xFF, ACCENT));
                cv.fill_ellipse(sr(((kx - 3.0) as i32, (cy - 3.0) as i32, 6, 6), s), argb(0xFF, WHITE));

                // Caixa de prévia.
                cv.fill_round(sr(PREVIEW[i], s), 4.0 * s, argb(0xFF, WHITE));
                cv.stroke_round(sr(PREVIEW[i], s), 4.0 * s, argb(0xFF, BORDER), s.max(1.0));
            }
            // Prévia da espessura: um ponto com o diâmetro escolhido, na cor atual.
            let pr = PREVIEW[0];
            let d = (slider(0) as f32).min(30.0) * s;
            let (cx, cy) = ((pr.0 as f32 + pr.2 as f32 / 2.0) * s, (pr.1 as f32 + pr.3 as f32 / 2.0) * s);
            let dot = (cx - d / 2.0, cy - d / 2.0, d, d);
            cv.fill_ellipse(dot, argb_from_colorref(color.colorref()));

            // Toggle "Start with Windows".
            let t = sr(TOGGLE, s);
            cv.fill_round(t, 11.0 * s, argb(0xFF, if on { ACCENT } else { CHIP_BORDER }));
            let kx = if on { TOGGLE.0 + 20 } else { TOGGLE.0 + 2 };
            cv.fill_ellipse(sr((kx, TOGGLE.1 + 2, 18, 18), s), argb(0xFF, WHITE));

            // Seletores de formato e estilo.
            for (i, r) in FMT_CHIPS.iter().enumerate() {
                chip(&cv, *r, s, i == fmt);
            }
            for (i, r) in STYLE_CHIPS.iter().enumerate() {
                chip(&cv, *r, s, i == style);
            }

            // Botões.
            cv.fill_round(sr(CANCEL_BTN, s), 8.0 * s, argb(0xFF, WHITE));
            cv.stroke_round(sr(CANCEL_BTN, s), 8.0 * s, argb(0xFF, BORDER), s.max(1.0));
            cv.fill_round(sr(SAVE_BTN, s), 8.0 * s, argb(0xFF, ACCENT));
        }

        // Textos.
        let dark = cref(INK);
        text(hdc, (32, 26, 300, 24), s, "Settings", 18, true, dark, DT_LEFT);
        for (label, y) in [("CAPTURE HOTKEY", 80), ("DEFAULT COLOR", 172), ("STROKE WIDTH", 226), ("TEXT SIZE", 268), ("SAVE FORMAT", 320), ("TOOLBAR STYLE", 378)] {
            text(hdc, (32, y, 200, 16), s, label, 11, true, cref(MUTED), DT_LEFT);
        }
        for (i, (_, name)) in HOTKEYS.iter().enumerate() {
            text(hdc, HK_CHIPS[i], s, name, 11, i == hk, cref(if i == hk { WHITE } else { CHIP_OFF_INK }), DT_CENTER);
        }
        if hk != CUSTOM_INDEX {
            text(hdc, (42, 136, 340, 24), s, "Enabled when Custom is selected", 12, false, cref(PLACEHOLDER), DT_LEFT);
        }
        text(hdc, (200, 172, 200, 16), s, COLORS[col].1, 12, false, cref(MUTED), DT_RIGHT);
        text(hdc, VALUE_TEXT[0], s, &format!("{} px", slider(0)), 12, true, dark, DT_LEFT);
        text(hdc, VALUE_TEXT[1], s, &format!("{} pt", slider(1)), 12, true, dark, DT_LEFT);
        let sample_px = (slider(1) as i32).min(28);
        text(hdc, PREVIEW[1], s, "ABC", sample_px, true, argb_to_cref(color.colorref()), DT_CENTER);
        for (i, (_, name)) in FORMATS.iter().enumerate() {
            text(hdc, FMT_CHIPS[i], s, name, 12, i == fmt, cref(if i == fmt { WHITE } else { CHIP_OFF_INK }), DT_CENTER);
        }
        for (i, (_, name)) in STYLES.iter().enumerate() {
            text(hdc, STYLE_CHIPS[i], s, name, 12, i == style, cref(if i == style { WHITE } else { CHIP_OFF_INK }), DT_CENTER);
        }
        text(hdc, TOGGLE_LABEL, s, "Start with Windows", 13, false, dark, DT_LEFT);
        text(hdc, CANCEL_BTN, s, "Cancel", 13, true, dark, DT_CENTER);
        text(hdc, SAVE_BTN, s, "Save", 13, true, cref(WHITE), DT_CENTER);

        let _ = EndPaint(hwnd, &ps);
    }
}

/// Já vem em COLORREF (0x00BBGGRR) de `Color::colorref`.
fn argb_to_cref(c: u32) -> COLORREF {
    COLORREF(c)
}

/// Chip segmentado: roxo quando escolhido; cinza com borda quando não.
fn chip(cv: &Canvas, r: (i32, i32, i32, i32), s: f32, selected: bool) {
    if selected {
        cv.fill_round(sr(r, s), 4.0 * s, argb(0xFF, ACCENT));
    } else {
        cv.fill_round(sr(r, s), 4.0 * s, argb(0xFF, CHIP_OFF));
        cv.stroke_round(sr(r, s), 4.0 * s, argb(0xFF, CHIP_BORDER), s.max(1.0));
    }
}

/// Clique (no botão do mouse já solto) em qualquer controle desenhado.
fn click(hwnd: HWND, p: (f32, f32)) {
    if let Some(i) = first_hit(p, &HK_CHIPS) {
        HK.store(i, Ordering::Relaxed);
        sync_custom(hwnd);
        redraw(hwnd);
        return;
    }
    if let Some(i) = (0..COLORS.len()).find(|&i| inside(p, (32 + i as i32 * SWATCH_STEP, SWATCH_Y, SWATCH, SWATCH))) {
        COLOR.store(i, Ordering::Relaxed);
        redraw(hwnd);
        return;
    }
    if let Some(i) = first_hit(p, &FMT_CHIPS) {
        FMT.store(i, Ordering::Relaxed);
        redraw(hwnd);
        return;
    }
    if let Some(i) = first_hit(p, &STYLE_CHIPS) {
        STYLE.store(i, Ordering::Relaxed);
        redraw(hwnd);
        return;
    }
    if inside(p, TOGGLE) || inside(p, TOGGLE_LABEL) {
        let v = STARTUP.load(Ordering::Relaxed);
        STARTUP.store(!v, Ordering::Relaxed);
        redraw(hwnd);
        return;
    }
    if inside(p, SAVE_BTN) {
        if save(hwnd) {
            close(hwnd);
        }
        return;
    }
    if inside(p, CANCEL_BTN) {
        close(hwnd);
    }
}

fn close(hwnd: HWND) {
    unsafe {
        let _ = DestroyWindow(hwnd);
    }
}

unsafe extern "system" fn proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            paint(hwnd);
            LRESULT(0)
        }
        WM_ERASEBKGND => LRESULT(1),
        WM_NCHITTEST => {
            // Sem barra de título: o topo da janela arrasta, o resto é cliente.
            let mut pt = POINT { x: (lp.0 & 0xFFFF) as i16 as i32, y: ((lp.0 >> 16) & 0xFFFF) as i16 as i32 };
            unsafe {
                let _ = ScreenToClient(hwnd, &mut pt);
            }
            let y = pt.y as f32 / scale();
            if y < HEADER_H as f32 {
                return LRESULT(HTCAPTION);
            }
            if y < HEADER_H as f32 {
                return LRESULT(HTCAPTION);
            }
            unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
        }
        WM_LBUTTONDOWN => {
            let p = mouse_xy(lp);
            if let Some(i) = (0..2).find(|&i| {
                p.0 >= TRACK_X0 - 8.0 && p.0 <= TRACK_X1 + 8.0 && (p.1 - TRACK_Y[i]).abs() <= 12.0
            }) {
                DRAG.store(i as u32, Ordering::Relaxed);
                unsafe {
                    SetCapture(hwnd);
                }
                set_from_x(hwnd, i, p.0);
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let d = DRAG.load(Ordering::Relaxed) as usize;
            if d < 2 {
                set_from_x(hwnd, d, mouse_xy(lp).0);
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            if DRAG.swap(9, Ordering::Relaxed) < 2 {
                unsafe {
                    let _ = ReleaseCapture();
                }
                return LRESULT(0);
            }
            click(hwnd, mouse_xy(lp));
            LRESULT(0)
        }
        WM_KEYDOWN => {
            match wp.0 {
                VK_RETURN_KEY => {
                    if save(hwnd) {
                        close(hwnd);
                    }
                }
                VK_ESCAPE_KEY => close(hwnd),
                _ => {}
            }
            LRESULT(0)
        }
        WM_CLOSE => {
            close(hwnd);
            LRESULT(0)
        }
        WM_DESTROY => LRESULT(0),
        _ => unsafe { DefWindowProcW(hwnd, msg, wp, lp) },
    }
}

/// Grava as escolhas, registra o atalho e ajusta a inicialização. Verdadeiro se salvou.
fn save(hwnd: HWND) -> bool {
    let path = crate::settings_path();
    let mut s = Settings::load(&path);
    let (hk, _) = HOTKEYS[HK.load(Ordering::Relaxed).min(HOTKEYS.len() - 1)];
    s.hotkey = hk;
    if s.hotkey == HotkeyChoice::Custom {
        let custom = unsafe { GetDlgItem(Some(hwnd), ID_CUSTOM).unwrap_or_default() };
        let h = from_control(send(custom, WM_GETHOTKEY, WPARAM(0), LPARAM(0)).0 as usize);
        if let Err(msg) = valid_custom(h) {
            info(hwnd, msg);
            return false;
        }
        s.custom_hotkey = Some(h);
    }
    s.default_color = COLORS[COLOR.load(Ordering::Relaxed).min(COLORS.len() - 1)].0;
    s.save_format = FORMATS[FMT.load(Ordering::Relaxed).min(FORMATS.len() - 1)].0;
    s.ui_style = STYLES[STYLE.load(Ordering::Relaxed).min(STYLES.len() - 1)].0;
    s.start_with_windows = STARTUP.load(Ordering::Relaxed);
    s.stroke_width = slider(0);
    s.text_size = slider(1);

    if s.save(&path).is_err() {
        info(hwnd, "Could not save the settings.");
        return false;
    }
    let want = s.start_with_windows;
    std::thread::spawn(move || {
        let _ = startup::set_enabled(want);
    });
    let main = HWND(MAIN.load(Ordering::Relaxed) as *mut _);
    let (_, warn) = hotkey::register(main, &s);
    if let Some(w) = warn {
        info(hwnd, w);
    }
    true
}

fn info(hwnd: HWND, text: &str) {
    let t = wide(text);
    unsafe {
        MessageBoxW(Some(hwnd), PCWSTR(t.as_ptr()), w!("Screenshotter"), MB_OK | MB_ICONINFORMATION);
    }
}

/// Abre a janela (ou traz a existente para frente).
pub fn open(main: HWND) {
    MAIN.store(main.0 as isize, Ordering::Relaxed);
    unsafe {
        if let Ok(existing) = FindWindowW(CLASS, None) {
            let _ = SetForegroundWindow(existing);
            return;
        }
    }
    let p = crate::settings_path();
    let mut cur = Settings::load(&p);
    // Sem arquivo ainda: o estado real de "iniciar com o Windows" vem do registro.
    if !p.exists() {
        cur.start_with_windows = startup::is_enabled();
    }
    HK.store(HOTKEYS.iter().position(|(c, _)| *c == cur.hotkey).unwrap_or(0), Ordering::Relaxed);
    COLOR.store(COLORS.iter().position(|(c, _)| *c == cur.default_color).unwrap_or(6), Ordering::Relaxed);
    FMT.store(FORMATS.iter().position(|(c, _)| *c == cur.save_format).unwrap_or(0), Ordering::Relaxed);
    STYLE.store(STYLES.iter().position(|(c, _)| *c == cur.ui_style).unwrap_or(0), Ordering::Relaxed);
    STARTUP.store(cur.start_with_windows, Ordering::Relaxed);
    VALS[0].store(cur.stroke_width.clamp(STROKE_RANGE.0, STROKE_RANGE.1), Ordering::Relaxed);
    VALS[1].store(cur.text_size.clamp(TEXT_RANGE.0, TEXT_RANGE.1), Ordering::Relaxed);
    DRAG.store(9, Ordering::Relaxed);

    unsafe {
        let hinst = GetModuleHandleW(None).unwrap_or_default();
        let wc = WNDCLASSW {
            lpfnWndProc: Some(proc),
            hInstance: hinst.into(),
            lpszClassName: CLASS,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            ..Default::default()
        };
        RegisterClassW(&wc);

        // Registra a classe do campo de tecla (msctls_hotkey32).
        let icc = INITCOMMONCONTROLSEX {
            dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dwICC: ICC_HOTKEY_CLASS,
        };
        let _ = InitCommonControlsEx(&icc);

        let s = (GetDpiForSystem() as f32 / 96.0).max(1.0);
        SCALE.store(s.to_bits(), Ordering::Relaxed);
        let sc = |v: i32| (v as f32 * s).round() as i32;
        let (ww, wh) = (sc(W), sc(H));
        // Centralizada na área de trabalho (acima da barra de tarefas), como no Lightshot.
        let mut wa = RECT::default();
        let _ = SystemParametersInfoW(
            SYSTEM_PARAMETERS_INFO_ACTION(48), // SPI_GETWORKAREA
            0,
            Some(&mut wa as *mut RECT as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        let x = wa.left + (wa.right - wa.left - ww) / 2;
        let y = wa.top + (wa.bottom - wa.top - wh) / 2;

        let title = wide("Screenshotter — Settings");
        // Janela sem moldura padrão: o desenho traz o card, a borda e os cantos arredondados.
        let hwnd = match CreateWindowExW(
            WINDOW_EX_STYLE(0x0000_0080), // WS_EX_TOOLWINDOW: sem botão na barra de tarefas
            CLASS,
            PCWSTR(title.as_ptr()),
            WS_POPUP,
            x,
            y,
            ww,
            wh,
            None,
            None,
            Some(hinst.into()),
            None,
        ) {
            Ok(h) => h,
            Err(_) => return,
        };
        let _ = SetWindowRgn(hwnd, Some(CreateRoundRectRgn(0, 0, ww + 1, wh + 1, sc(16), sc(16))), true);

        let font = CreateFontW(
            -sc(12),
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            FONT_CHARSET(1),
            FONT_OUTPUT_PRECISION(0),
            FONT_CLIP_PRECISION(0),
            FONT_QUALITY(5),
            0,
            w!("Segoe UI"),
        );
        let custom = child(hwnd, w!("msctls_hotkey32"), "", WS_TABSTOP.0, 0, ID_CUSTOM, CUSTOM_FIELD, s, font.0 as usize);
        if let Some(h) = cur.custom_hotkey {
            send(custom, WM_SETHOTKEY, WPARAM(to_control(h)), LPARAM(0));
        }
        sync_custom(hwnd);

        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = SetForegroundWindow(hwnd);
    }
}

/// Controle nativo filho, posicionado em coordenadas lógicas.
fn child(
    parent: HWND,
    class: PCWSTR,
    text: &str,
    style: u32,
    ex: u32,
    id: i32,
    r: (i32, i32, i32, i32),
    s: f32,
    font: usize,
) -> HWND {
    let rc = rect_px(r, s);
    let t = wide(text);
    unsafe {
        let hinst = GetModuleHandleW(None).unwrap_or_default();
        let c = CreateWindowExW(
            WINDOW_EX_STYLE(ex),
            class,
            PCWSTR(t.as_ptr()),
            WS_CHILD | WS_VISIBLE | WINDOW_STYLE(style),
            rc.left,
            rc.top,
            rc.right - rc.left,
            rc.bottom - rc.top,
            Some(parent),
            Some(HMENU(id as usize as *mut _)),
            Some(hinst.into()),
            None,
        )
        .unwrap_or_default();
        send(c, WM_SETFONT, WPARAM(font), LPARAM(1));
        c
    }
}
