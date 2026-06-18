//! CPU post-process pipeline for AI output and fast quantize (offline path).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessOptions {
    pub target_width: u32,
    pub target_height: u32,
    pub max_colors: u8,
    pub dither: bool,
    pub despeckle: bool,
    pub outline: OutlineMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutlineMode {
    #[default]
    None,
    Black1px,
    ProjectStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    fn from_slice(px: &[u8]) -> Self {
        Self {
            r: px[0],
            g: px[1],
            b: px[2],
            a: px[3],
        }
    }

    fn to_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    fn dist_sq(self, other: Rgba) -> u32 {
        let dr = i32::from(self.r) - i32::from(other.r);
        let dg = i32::from(self.g) - i32::from(other.g);
        let db = i32::from(self.b) - i32::from(other.b);
        let da = i32::from(self.a) - i32::from(other.a);
        (dr * dr + dg * dg + db * db + da * da) as u32
    }
}

/// Nearest-neighbor downscale → palette quantize → optional despeckle → optional outline.
pub fn process_rgba(
    rgba: &[u8],
    width: u32,
    height: u32,
    opts: &PostProcessOptions,
) -> Vec<u8> {
    let expected = (width * height * 4) as usize;
    if rgba.len() != expected || width == 0 || height == 0 {
        return Vec::new();
    }

    let scaled = nearest_downscale(rgba, width, height, opts.target_width, opts.target_height);
    let mut out = quantize(
        &scaled,
        opts.target_width,
        opts.target_height,
        opts.max_colors.max(2),
        opts.dither,
    );

    if opts.despeckle {
        despeckle(&mut out, opts.target_width, opts.target_height);
    }

    match opts.outline {
        OutlineMode::None => {}
        OutlineMode::Black1px | OutlineMode::ProjectStyle => {
            apply_outline(&mut out, opts.target_width, opts.target_height);
        }
    }

    out
}

fn nearest_downscale(src: &[u8], sw: u32, sh: u32, dw: u32, dh: u32) -> Vec<u8> {
    if dw == 0 || dh == 0 {
        return Vec::new();
    }
    if sw == dw && sh == dh {
        return src.to_vec();
    }

    let mut out = vec![0u8; (dw * dh * 4) as usize];
    for y in 0..dh {
        for x in 0..dw {
            let sx = (x * sw) / dw;
            let sy = (y * sh) / dh;
            let si = ((sy * sw + sx) * 4) as usize;
            let di = ((y * dw + x) * 4) as usize;
            out[di..di + 4].copy_from_slice(&src[si..si + 4]);
        }
    }
    out
}

fn quantize(rgba: &[u8], width: u32, height: u32, max_colors: u8, dither: bool) -> Vec<u8> {
    let count = (width * height) as usize;
    let mut pixels: Vec<Rgba> = Vec::with_capacity(count);
    for i in 0..count {
        pixels.push(Rgba::from_slice(&rgba[i * 4..i * 4 + 4]));
    }

    let mut palette = build_palette(&pixels, max_colors);
    if palette.is_empty() {
        palette.push(Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });
    }

    let mut out = vec![0u8; count * 4];
    for (i, px) in pixels.iter().enumerate() {
        let mapped = if px.a < 16 {
            palette[0]
        } else if dither {
            nearest_palette_color(*px, &palette)
        } else {
            nearest_palette_color(*px, &palette)
        };
        out[i * 4..i * 4 + 4].copy_from_slice(&mapped.to_bytes());
    }
    out
}

fn build_palette(pixels: &[Rgba], max_colors: u8) -> Vec<Rgba> {
    let mut counts: std::collections::HashMap<[u8; 4], u32> = std::collections::HashMap::new();
    for px in pixels {
        if px.a < 16 {
            continue;
        }
        *counts.entry(px.to_bytes()).or_insert(0) += 1;
    }

    let mut entries: Vec<(Rgba, u32)> = counts
        .into_iter()
        .map(|(k, c)| (Rgba::from_slice(&k), c))
        .collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));

    let mut palette: Vec<Rgba> = vec![Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }];
    for (color, _) in entries.into_iter().take(max_colors.saturating_sub(1) as usize) {
        if !palette.iter().any(|p| *p == color) {
            palette.push(color);
        }
    }

    while palette.len() < max_colors as usize && palette.len() < 256 {
        let mut best_pair = None;
        let mut best_dist = u32::MAX;
        for i in 1..palette.len() {
            for j in (i + 1)..palette.len() {
                let d = palette[i].dist_sq(palette[j]);
                if d < best_dist {
                    best_dist = d;
                    best_pair = Some((i, j));
                }
            }
        }
        let Some((i, j)) = best_pair else {
            break;
        };
        if palette.len() <= max_colors as usize {
            break;
        }
        let merged = Rgba {
            r: ((u16::from(palette[i].r) + u16::from(palette[j].r)) / 2) as u8,
            g: ((u16::from(palette[i].g) + u16::from(palette[j].g)) / 2) as u8,
            b: ((u16::from(palette[i].b) + u16::from(palette[j].b)) / 2) as u8,
            a: ((u16::from(palette[i].a) + u16::from(palette[j].a)) / 2) as u8,
        };
        palette[j] = merged;
        palette.remove(i);
    }

    palette
}

fn nearest_palette_color(px: Rgba, palette: &[Rgba]) -> Rgba {
    palette
        .iter()
        .copied()
        .min_by_key(|c| px.dist_sq(*c))
        .unwrap_or(palette[0])
}

fn despeckle(rgba: &mut [u8], width: u32, height: u32) {
    let w = width as i32;
    let h = height as i32;
    let original = rgba.to_vec();
    let neighbors = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 4) as usize;
            let center = &original[idx..idx + 4];
            let mut same = 0;
            for (dx, dy) in neighbors {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= w || ny >= h {
                    continue;
                }
                let ni = ((ny * w + nx) * 4) as usize;
                if &original[ni..ni + 4] == center {
                    same += 1;
                }
            }
            if same < 2 {
                let mut counts: std::collections::HashMap<[u8; 4], u8> =
                    std::collections::HashMap::new();
                for (dx, dy) in neighbors {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= w || ny >= h {
                        continue;
                    }
                    let ni = ((ny * w + nx) * 4) as usize;
                    let key = [
                        original[ni],
                        original[ni + 1],
                        original[ni + 2],
                        original[ni + 3],
                    ];
                    *counts.entry(key).or_insert(0) += 1;
                }
                if let Some((best, _)) = counts.iter().max_by_key(|(_, c)| *c) {
                    rgba[idx..idx + 4].copy_from_slice(best);
                }
            }
        }
    }
}

fn apply_outline(rgba: &mut [u8], width: u32, height: u32) {
    let w = width as i32;
    let h = height as i32;
    let original = rgba.to_vec();
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 4) as usize;
            if original[idx + 3] >= 16 {
                continue;
            }
            let mut touches_opaque = false;
            for (dx, dy) in neighbors {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= w || ny >= h {
                    continue;
                }
                let ni = ((ny * w + nx) * 4) as usize;
                if original[ni + 3] >= 16 {
                    touches_opaque = true;
                    break;
                }
            }
            if touches_opaque {
                rgba[idx] = 0;
                rgba[idx + 1] = 0;
                rgba[idx + 2] = 0;
                rgba[idx + 3] = 255;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(w: u32, h: u32, color: [u8; 4]) -> Vec<u8> {
        let mut buf = vec![0u8; (w * h * 4) as usize];
        for i in 0..(w * h) as usize {
            buf[i * 4..i * 4 + 4].copy_from_slice(&color);
        }
        buf
    }

    #[test]
    fn downscales_to_target_size() {
        let src = solid(64, 64, [200, 50, 50, 255]);
        let out = process_rgba(
            &src,
            64,
            64,
            &PostProcessOptions {
                target_width: 16,
                target_height: 16,
                max_colors: 8,
                dither: false,
                despeckle: false,
                outline: OutlineMode::None,
            },
        );
        assert_eq!(out.len(), 16 * 16 * 4);
        assert_eq!(out[0..4], [200, 50, 50, 255]);
    }

    #[test]
    fn respects_max_colors() {
        let mut src = vec![0u8; 8 * 8 * 4];
        for (i, px) in src.chunks_mut(4).enumerate() {
            let v = (i % 16) as u8 * 16;
            px.copy_from_slice(&[v, 255 - v, 128, 255]);
        }
        let out = process_rgba(
            &src,
            8,
            8,
            &PostProcessOptions {
                target_width: 8,
                target_height: 8,
                max_colors: 4,
                dither: false,
                despeckle: false,
                outline: OutlineMode::None,
            },
        );
        let mut unique = std::collections::HashSet::new();
        for px in out.chunks(4) {
            unique.insert([px[0], px[1], px[2], px[3]]);
        }
        assert!(unique.len() <= 4);
    }

    #[test]
    fn invalid_input_returns_empty() {
        let out = process_rgba(&[0, 0, 0], 2, 2, &PostProcessOptions {
            target_width: 1,
            target_height: 1,
            max_colors: 4,
            dither: false,
            despeckle: false,
            outline: OutlineMode::None,
        });
        assert!(out.is_empty());
    }
}
