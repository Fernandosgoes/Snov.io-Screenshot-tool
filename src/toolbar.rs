//! Barras de ícones (PRD F7) e paleta de 7 cores (F11).
//! Dois estilos (ver `UiStyle`): Conservadora, barra clara e arredondada com botões de 30 px
//! e destaque roxo; Moderna, painel escuro com botões circulares. Traço suave e arredondado.
//! Layout puro + pintura suavizada (GDI+, ver gfx.rs).

use crate::config::{Color, UiStyle};
use crate::geom::{BarLayout, GAP, Point, Rect, layout_bars, shift_inside};
use crate::gfx::{Canvas, RectF, argb, argb_from_colorref};
use std::f32::consts::{FRAC_PI_2, PI};

const SWATCH: i32 = 20;
const SWATCH_GAP: i32 = 5;

/// Medidas e cores de um estilo. Cores em 0xRRGGBB; os alfas são separados.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Look {
    /// Lado de cada botão, em pixels.
    pub btn: i32,
    /// Margem interna do painel em volta dos botões.
    pub pad: i32,
    pub panel_r: f32,
    pub btn_r: f32,
    pub panel_rgb: u32,
    pub panel_a: u8,
    pub border_rgb: u32,
    pub border_a: u8,
    pub ink_rgb: u32,
    pub ink_active_rgb: u32,
    pub accent_rgb: u32,
    pub hover_rgb: u32,
    pub hover_a: u8,
    /// Anel da cor ativa na paleta.
    pub ring_rgb: u32,
}

/// Medidas e cores de cada estilo. Conservadora é o padrão.
pub fn look(style: UiStyle) -> Look {
    match style {
        UiStyle::Conservadora => Look {
            btn: 30,
            pad: 4,
            panel_r: 12.0,
            btn_r: 8.0,
            panel_rgb: 0xFFFFFF,
            panel_a: 0xFF,
            border_rgb: 0xB9B2D8,
            border_a: 0xFF,
            ink_rgb: 0x2B2555,
            ink_active_rgb: 0xFFFFFF,
            accent_rgb: 0x6F55E8,
            hover_rgb: 0xF1EFF8,
            hover_a: 0xFF,
            ring_rgb: 0x2B2555,
        },
        UiStyle::Moderna => Look {
            btn: 30,
            pad: 4,
            panel_r: 12.0,
            btn_r: 8.0,
            panel_rgb: 0x241B45,
            panel_a: 0xF2,
            border_rgb: 0xFFFFFF,
            border_a: 0x30,
            ink_rgb: 0xD8CFFA,
            ink_active_rgb: 0xFFFFFF,
            accent_rgb: 0x7B5CF0,
            hover_rgb: 0xFFFFFF,
            hover_a: 0x26,
            ring_rgb: 0xFFFFFF,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Move,
    Pen,
    Line,
    Arrow,
    Rect,
    Ellipse,
    Marker,
    Text,
    Pixelate,
    Number,
    ColorSwatch,
    Undo,
    /// Borracha: apaga todas as anotações (ação, não ferramenta).
    ClearAll,
    Copy,
    Save,
    Close,
    /// Ícone de mover texto, que aparece ao lado do T (não faz parte das barras).
    TextMove,
}

/// Barra vertical, de cima para baixo. A borracha fica no topo, acima da cruz de mover.
pub const VERTICAL: [Button; 13] = [
    Button::ClearAll,
    Button::Move,
    Button::Pen,
    Button::Line,
    Button::Arrow,
    Button::Rect,
    Button::Ellipse,
    Button::Marker,
    Button::Text,
    Button::Pixelate,
    Button::Number,
    Button::ColorSwatch,
    Button::Undo,
];

pub const HORIZONTAL: [Button; 3] = [Button::Copy, Button::Save, Button::Close];

fn pad(r: Rect, p: i32) -> Rect {
    Rect { x: r.x - p, y: r.y - p, w: r.w + 2 * p, h: r.h + 2 * p }
}

/// Painéis (com margem) das duas barras, sempre inteiramente dentro de `screen`.
pub fn layout(sel: Rect, screen: Rect, k: &Look) -> BarLayout {
    let base = layout_bars(sel, screen, VERTICAL.len() as i32, HORIZONTAL.len() as i32, k.btn);
    let vertical = shift_inside(pad(base.vertical, k.pad), screen);
    // Ícone de mover texto: do lado de fora da barra, na linha do T; do lado de dentro só se
    // não couber fora da tela.
    let row = VERTICAL.iter().position(|b| *b == Button::Text).unwrap_or(7) as i32;
    let size = k.btn + 2 * k.pad;
    let y = vertical.y + k.pad + row * k.btn - k.pad;
    let outside_x = vertical.right() + GAP;
    let x = if outside_x + size <= screen.right() { outside_x } else { (vertical.x - GAP - size).max(0) };
    BarLayout {
        vertical,
        horizontal: shift_inside(pad(base.horizontal, k.pad), screen),
        text_move: Rect { x, y, w: size, h: size },
    }
}

/// Retângulo de cada botão, na ordem de `VERTICAL` e `HORIZONTAL`.
pub fn button_rects(l: &BarLayout, k: &Look) -> (Vec<(Button, Rect)>, Vec<(Button, Rect)>) {
    let v = VERTICAL
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (
                *b,
                Rect { x: l.vertical.x + k.pad, y: l.vertical.y + k.pad + i as i32 * k.btn, w: k.btn, h: k.btn },
            )
        })
        .collect();
    let h = HORIZONTAL
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (
                *b,
                Rect { x: l.horizontal.x + k.pad + i as i32 * k.btn, y: l.horizontal.y + k.pad, w: k.btn, h: k.btn },
            )
        })
        .collect();
    (v, h)
}

pub fn hit(l: &BarLayout, p: Point, k: &Look) -> Option<Button> {
    if text_move_panel(l, k).contains(p) {
        return Some(Button::TextMove);
    }
    let (v, h) = button_rects(l, k);
    v.into_iter().chain(h).find(|(_, r)| r.contains(p)).map(|(b, _)| b)
}

/// Painel do ícone de mover texto: fica à esquerda da barra, na linha do T.
pub fn text_move_panel(l: &BarLayout, _k: &Look) -> Rect {
    l.text_move
}

/// Retângulo do botão de mover texto, dentro do seu painel.
pub fn text_move_button(panel: Rect, k: &Look) -> Rect {
    Rect { x: panel.x + k.pad, y: panel.y + k.pad, w: k.btn, h: k.btn }
}

/// Painel da paleta: abre à esquerda da barra vertical, na linha do quadradinho de cor.
pub fn palette_panel(l: &BarLayout, colors: &[Color], k: &Look) -> Rect {
    let row = VERTICAL.iter().position(|b| *b == Button::ColorSwatch).unwrap_or(9) as i32;
    let w = colors.len() as i32 * (SWATCH + SWATCH_GAP) - SWATCH_GAP + 2 * k.pad;
    let h = SWATCH + 2 * k.pad;
    let center_y = l.vertical.y + k.pad + row * k.btn + k.btn / 2;
    Rect { x: (l.vertical.x - GAP - w).max(0), y: center_y - h / 2, w, h }
}

pub fn palette_rects(l: &BarLayout, colors: &[Color], k: &Look) -> Vec<(Color, Rect)> {
    let p = palette_panel(l, colors, k);
    colors
        .iter()
        .enumerate()
        .map(|(i, c)| {
            (*c, Rect { x: p.x + k.pad + i as i32 * (SWATCH + SWATCH_GAP), y: p.y + k.pad, w: SWATCH, h: SWATCH })
        })
        .collect()
}

pub fn palette_hit(l: &BarLayout, colors: &[Color], p: Point, k: &Look) -> Option<Color> {
    palette_rects(l, colors, k).into_iter().find(|(_, r)| r.contains(p)).map(|(c, _)| c)
}

fn rf(r: Rect) -> RectF {
    (r.x as f32, r.y as f32, r.w as f32, r.h as f32)
}

/// Fundo do painel: sombra suave, preenchimento e borda do estilo.
pub fn paint_panel(cv: &Canvas, panel: Rect, k: &Look) {
    let r = rf(panel);
    cv.shadow(r, k.panel_r);
    cv.fill_round(r, k.panel_r, argb(k.panel_a, k.panel_rgb));
    cv.stroke_round(
        (r.0 + 0.5, r.1 + 0.5, r.2 - 1.0, r.3 - 1.0),
        k.panel_r - 0.5,
        argb(k.border_a, k.border_rgb),
        1.0,
    );
}

/// Pinta os botões de uma barra: destaque no ativo, realce no hover, ícones.
pub fn paint_buttons(
    cv: &Canvas,
    buttons: &[(Button, Rect)],
    active: Option<Button>,
    hover: Option<Button>,
    current: Color,
    k: &Look,
) {
    for (b, r) in buttons {
        let is_active = Some(*b) == active;
        let rr = rf(*r);
        let inner = (rr.0 + 1.0, rr.1 + 1.0, rr.2 - 2.0, rr.3 - 2.0);
        if is_active {
            cv.fill_round(inner, k.btn_r, argb(0xFF, k.accent_rgb));
        } else if Some(*b) == hover {
            cv.fill_round(inner, k.btn_r, argb(k.hover_a, k.hover_rgb));
        }
        draw_icon(cv, *b, *r, current, is_active, k);
    }
}

/// Paleta: bolinhas com a cor; a cor ativa tem anel de destaque.
pub fn paint_palette(cv: &Canvas, panel: Rect, swatches: &[(Color, Rect)], current: Color, k: &Look) {
    paint_panel(cv, panel, k);
    for (c, r) in swatches {
        let rr = rf(*r);
        cv.fill_ellipse(rr, argb_from_colorref(c.colorref()));
        // Contorno sempre visível (o branco some sem ele).
        cv.stroke_ellipse(rr, argb(0xFF, k.border_rgb), 1.5);
        if *c == current {
            cv.stroke_ellipse((rr.0 - 3.5, rr.1 - 3.5, rr.2 + 7.0, rr.3 + 7.0), argb(0xFF, k.ring_rgb), 2.0);
        }
    }
}

/// Pontos de uma curva cúbica de Bézier (grade de ícones), amostrados em 8 passos.
fn cubic(c: [(f32, f32); 4]) -> Vec<(f32, f32)> {
    (0..=8)
        .map(|i| {
            let t = i as f32 / 8.0;
            let u = 1.0 - t;
            let (a, b, cc, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
            (
                a * c[0].0 + b * c[1].0 + cc * c[2].0 + d * c[3].0,
                a * c[0].1 + b * c[1].1 + cc * c[2].1 + d * c[3].1,
            )
        })
        .collect()
}

/// Pontos de um arco de círculo entre os ângulos `a0` e `a1` (radianos).
fn arc(cx: f32, cy: f32, r: f32, a0: f32, a1: f32, n: usize) -> Vec<(f32, f32)> {
    (0..=n)
        .map(|i| {
            let t = a0 + (a1 - a0) * i as f32 / n as f32;
            (cx + r * t.cos(), cy + r * t.sin())
        })
        .collect()
}

/// Ícones da mockup: desenhados numa grade de 24 unidades, traço suave e pontas arredondadas.
/// A grade é escalada para ~60% do botão.
fn draw_icon(cv: &Canvas, b: Button, r: Rect, current: Color, active: bool, k: &Look) {
    let cx = r.x as f32 + r.w as f32 / 2.0;
    let cy = r.y as f32 + r.h as f32 / 2.0;
    let sc = k.btn as f32 * 0.6 / 24.0;
    let rgb = if active { k.ink_active_rgb } else { k.ink_rgb };
    let ink = argb(0xFF, rgb);
    let w = (1.5 * sc * 1.25).max(1.1);
    let p = |x: f32, y: f32| (cx + (x - 12.0) * sc, cy + (y - 12.0) * sc);
    // Retângulo (x, y, largura, altura) na grade, convertido para pixels.
    let q = |x: f32, y: f32, ww: f32, hh: f32| {
        let (px, py) = p(x, y);
        (px, py, ww * sc, hh * sc)
    };
    let m = |v: &[(f32, f32)]| -> Vec<(f32, f32)> { v.iter().map(|&(x, y)| p(x, y)).collect() };
    match b {
        Button::Move | Button::TextMove => {
            cv.line(p(12.0, 2.0), p(12.0, 22.0), ink, w);
            cv.line(p(2.0, 12.0), p(22.0, 12.0), ink, w);
            cv.polyline(&[p(9.0, 5.0), p(12.0, 2.0), p(15.0, 5.0)], ink, w);
            cv.polyline(&[p(9.0, 19.0), p(12.0, 22.0), p(15.0, 19.0)], ink, w);
            cv.polyline(&[p(5.0, 9.0), p(2.0, 12.0), p(5.0, 15.0)], ink, w);
            cv.polyline(&[p(19.0, 9.0), p(22.0, 12.0), p(19.0, 15.0)], ink, w);
        }
        Button::Pen => {
            let mut wave = cubic([(3.0, 14.0), (5.0, 10.0), (7.0, 10.0), (9.0, 14.0)]);
            wave.extend(cubic([(9.0, 14.0), (11.0, 18.0), (13.0, 18.0), (15.0, 14.0)]));
            wave.extend(cubic([(15.0, 14.0), (17.0, 10.0), (18.0, 11.0), (21.0, 13.0)]));
            cv.polyline(&m(&wave), ink, w);
        }
        Button::Line => cv.line(p(5.0, 19.0), p(19.0, 5.0), ink, w),
        Button::Arrow => {
            cv.line(p(5.0, 19.0), p(19.0, 5.0), ink, w);
            cv.polyline(&[p(9.0, 5.0), p(19.0, 5.0), p(19.0, 15.0)], ink, w);
        }
        Button::Rect => cv.stroke_round(q(3.0, 5.0, 18.0, 14.0), 3.0 * sc, ink, w),
        Button::Ellipse => cv.stroke_ellipse(q(3.0, 5.0, 18.0, 14.0), ink, w),
        Button::Marker => cv.fill_round(q(4.0, 9.5, 16.0, 5.0), 2.0 * sc, argb(0xDD, 0xFACC15)),
        Button::Text => {
            cv.polyline(&[p(5.0, 6.0), p(5.0, 4.0), p(19.0, 4.0), p(19.0, 6.0)], ink, w);
            cv.line(p(12.0, 4.0), p(12.0, 20.0), ink, w);
            cv.line(p(9.0, 20.0), p(15.0, 20.0), ink, w);
        }
        Button::Pixelate => {
            for gy in 0..3 {
                for gx in 0..3 {
                    let alpha = if (gx + gy) % 2 == 0 { 0xFF } else { 0x66 };
                    let sq = q(4.0 + gx as f32 * 6.0, 4.0 + gy as f32 * 6.0, 4.0, 4.0);
                    cv.fill_round(sq, 1.0 * sc, argb(alpha, rgb));
                }
            }
        }
        Button::Number => {
            // Círculo grande e um "1" menor, com folga dentro do círculo.
            cv.stroke_ellipse(q(2.0, 2.0, 20.0, 20.0), ink, w);
            cv.line(p(12.0, 8.0), p(12.0, 16.0), ink, w);
            cv.line(p(10.0, 9.8), p(12.0, 8.3), ink, w);
        }
        Button::ColorSwatch => {
            let dot = q(4.5, 4.5, 15.0, 15.0);
            cv.fill_ellipse(dot, argb_from_colorref(current.colorref()));
            cv.stroke_ellipse(dot, argb(0x66, 0xFFFFFF), 1.0);
        }
        Button::Undo => {
            cv.polyline(&[p(9.0, 14.0), p(4.0, 9.0), p(9.0, 4.0)], ink, w);
            cv.line(p(4.0, 9.0), p(14.0, 9.0), ink, w);
            let mut hook = arc(14.0, 15.0, 6.0, -FRAC_PI_2, FRAC_PI_2, 8);
            hook.push((11.0, 21.0));
            cv.polyline(&m(&hook), ink, w);
        }
        Button::ClearAll => {
            // Borracha da mockup: contorno da peça e traço de "apagar" embaixo.
            cv.polyline(&m(&[(3.0, 17.0), (11.0, 9.0), (17.0, 15.0), (13.0, 19.0), (7.0, 19.0), (3.0, 17.0)]), ink, w);
            cv.line(p(14.0, 21.0), p(21.0, 21.0), ink, w);
        }
        Button::Copy => {
            // Duas folhas: a da frente fechada e a de trás só nas bordas visíveis.
            cv.stroke_round(q(8.0, 8.0, 13.0, 13.0), 2.0 * sc, ink, w);
            let mut back = vec![(4.0, 16.0), (4.0, 6.0)];
            back.extend(arc(6.0, 6.0, 2.0, PI, 1.5 * PI, 4));
            back.push((16.0, 4.0));
            cv.polyline(&m(&back), ink, w);
        }
        Button::Save => {
            // Seta descendo para uma bandeja, como o ícone de salvar da marca.
            cv.polyline(&m(&[(4.0, 15.0), (4.0, 19.0), (20.0, 19.0), (20.0, 15.0)]), ink, w);
            cv.line(p(12.0, 3.0), p(12.0, 15.0), ink, w);
            cv.polyline(&m(&[(7.5, 10.5), (12.0, 15.0), (16.5, 10.5)]), ink, w);
        }
        Button::Close => {
            cv.line(p(6.0, 6.0), p(18.0, 18.0), ink, w);
            cv.line(p(18.0, 6.0), p(6.0, 18.0), ink, w);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STYLES: [UiStyle; 2] = [UiStyle::Conservadora, UiStyle::Moderna];

    #[test]
    fn panels_fit_inside_screen() {
        let screen = Rect { x: 0, y: 0, w: 1920, h: 1080 };
        let sel = Rect { x: 1800, y: 900, w: 100, h: 100 };
        for st in STYLES {
            let l = layout(sel, screen, &look(st));
            assert!(l.vertical.right() <= screen.right());
            assert!(l.vertical.bottom() <= screen.bottom());
            assert!(l.horizontal.right() <= screen.right());
            assert!(l.horizontal.bottom() <= screen.bottom());
        }
    }

    #[test]
    fn tall_bar_near_top_keeps_full_height() {
        // Seleção perto do topo: a barra vertical não pode ser encolhida.
        let screen = Rect { x: 0, y: 0, w: 1000, h: 800 };
        let sel = Rect { x: 100, y: 20, w: 200, h: 50 };
        for st in STYLES {
            let k = look(st);
            let l = layout(sel, screen, &k);
            let needed = VERTICAL.len() as i32 * k.btn + 2 * k.pad;
            assert_eq!(l.vertical.h, needed);
            assert!(l.vertical.y >= 0 && l.vertical.bottom() <= screen.bottom());
        }
    }

    #[test]
    fn buttons_stay_inside_their_panel() {
        let screen = Rect { x: 0, y: 0, w: 1000, h: 800 };
        let sel = Rect { x: 100, y: 100, w: 200, h: 200 };
        for st in STYLES {
            let k = look(st);
            let l = layout(sel, screen, &k);
            let (v, h) = button_rects(&l, &k);
            for (_, r) in &v {
                assert!(r.x >= l.vertical.x && r.right() <= l.vertical.right());
                assert!(r.y >= l.vertical.y && r.bottom() <= l.vertical.bottom());
            }
            for (_, r) in &h {
                assert!(r.x >= l.horizontal.x && r.right() <= l.horizontal.right());
                assert!(r.y >= l.horizontal.y && r.bottom() <= l.horizontal.bottom());
            }
        }
    }

    #[test]
    fn hit_returns_correct_button() {
        let screen = Rect { x: 0, y: 0, w: 1000, h: 800 };
        let sel = Rect { x: 100, y: 100, w: 200, h: 200 };
        for st in STYLES {
            let k = look(st);
            let l = layout(sel, screen, &k);
            let (v, _) = button_rects(&l, &k);
            let (b, r) = v[4];
            assert_eq!(hit(&l, Point { x: r.x + 1, y: r.y + 1 }, &k), Some(b));
            assert_eq!(b, Button::Arrow);
        }
    }

    #[test]
    fn clear_all_is_the_first_vertical_button() {
        assert_eq!(VERTICAL[0], Button::ClearAll);
        assert_eq!(VERTICAL[1], Button::Move);
    }

    #[test]
    fn palette_hit_maps_every_color() {
        let screen = Rect { x: 0, y: 0, w: 1000, h: 800 };
        let sel = Rect { x: 300, y: 300, w: 200, h: 200 };
        for st in STYLES {
            let k = look(st);
            let l = layout(sel, screen, &k);
            let rects = palette_rects(&l, &Color::ALL, &k);
            assert_eq!(rects.len(), Color::ALL.len());
            for (c, r) in rects {
                assert_eq!(palette_hit(&l, &Color::ALL, Point { x: r.x + 2, y: r.y + 2 }, &k), Some(c));
            }
        }
    }

    #[test]
    fn conservadora_is_light_and_moderna_is_dark() {
        // Luminância aproximada do painel: o claro deve ser mais brilhante que o escuro.
        let lum = |rgb: u32| ((rgb >> 16) & 0xFF) + ((rgb >> 8) & 0xFF) + (rgb & 0xFF);
        assert!(lum(look(UiStyle::Conservadora).panel_rgb) > lum(look(UiStyle::Moderna).panel_rgb));
        assert_eq!(look(UiStyle::Conservadora).btn, 30);
    }
}
