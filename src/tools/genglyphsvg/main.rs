use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::Deref;

use clap::Parser;
use read_fonts::TableProvider as _;
use skrifa::color::{ColorGlyph, ColorPainter, Transform};
use skrifa::instance::Size;
use skrifa::metrics::BoundingBox;
use skrifa::outline::pen::{OutlinePen, SvgPen, ControlBoundsPen};
use skrifa::prelude::LocationRef;
use skrifa::string::StringId;
use skrifa::{FontRef, GlyphId, MetadataProvider, OutlineGlyph};
use svg::Node as SvgNode;

enum OutlineGlyphOrBoundingBox<'a> {
    Glyph(Box<OutlineGlyph<'a>>),
    BBox(BoundingBox),
}

impl<'a> OutlineGlyphOrBoundingBox<'a> {
    fn draw(&self, g: &mut impl OutlinePen) {
        match self {
            OutlineGlyphOrBoundingBox::Glyph(glyph) => {
                glyph.draw(Size::unscaled(), g).unwrap();
            }
            OutlineGlyphOrBoundingBox::BBox(bbox) => {
                g.move_to(bbox.x_min, bbox.y_min);
                g.line_to(bbox.x_max, bbox.y_min);
                g.line_to(bbox.x_max, bbox.y_max);
                g.line_to(bbox.x_min, bbox.y_max);
                g.close();
            }
        }
    }
}

impl<'a> From<OutlineGlyph<'a>> for OutlineGlyphOrBoundingBox<'a> {
    fn from(glyph: OutlineGlyph<'a>) -> Self {
        Self::Glyph(Box::new(glyph))
    }
}

impl<'a> From<BoundingBox> for OutlineGlyphOrBoundingBox<'a> {
    fn from(bbox: BoundingBox) -> Self {
        Self::BBox(bbox)
    }
}

struct CountrolBoundsPainter<'a, 'b> {
    font: &'a FontRef<'b>,
    transforms: Vec<Transform>,
    clips: Vec<OutlineGlyphOrBoundingBox<'b>>,
    bbox: BoundingBox,
}

impl<'a, 'b> CountrolBoundsPainter<'a, 'b> {
    fn new(font: &'a FontRef<'b>) -> Self {
        Self {
            font,
            transforms: vec![Transform::default()],
            clips: Vec::new(),
            bbox: BoundingBox {
                x_min: f32::INFINITY,
                x_max: f32::NEG_INFINITY,
                y_min: f32::INFINITY,
                y_max: f32::NEG_INFINITY,
            },
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    fn current_transform(&self) -> &Transform {
        self.transforms.last().unwrap()
    }

    fn transform(&self, p: (f32, f32)) -> (f32, f32) {
        let t = self.current_transform();
        (
            t.xx * p.0 + t.xy * p.1 + t.dx,
            t.yx * p.0 + t.yy * p.1 + t.dy,
        )
    }
}

impl<'a, 'b> ColorPainter for CountrolBoundsPainter<'a, 'b> {
    fn push_transform(&mut self, tx: Transform) {
        self.transforms
            .push(*self.transforms.last().unwrap() * tx);
    }

    fn pop_transform(&mut self) {
        self.transforms.pop();
    }

    fn push_clip_glyph(&mut self, glyph_id: GlyphId) {
        self.clips
            .push(self.font.outline_glyphs().get(glyph_id).unwrap().into());
    }

    fn push_clip_box(&mut self, _clip_box: BoundingBox) {
        self.clips.push(_clip_box.into());
    }

    fn pop_clip(&mut self) {
        self.clips.pop();
    }

    fn fill(&mut self, _brush: skrifa::color::Brush<'_>) {
        let mut g = ControlBoundsPen::new();
        for clip in &self.clips {
            clip.draw(&mut g);
        }
        let bbox = g.bounding_box().unwrap();
        let p1 = self.transform((bbox.x_min, bbox.y_min));
        let p2 = self.transform((bbox.x_max, bbox.y_min));
        let p3 = self.transform((bbox.x_max, bbox.y_max));
        let p4 = self.transform((bbox.x_min, bbox.y_max));
        self.bbox.x_min = self.bbox.x_min.min(p1.0).min(p2.0).min(p3.0).min(p4.0);
        self.bbox.x_max = self.bbox.x_max.max(p1.0).max(p2.0).max(p3.0).max(p4.0);
        self.bbox.y_min = self.bbox.y_min.min(p1.1).min(p2.1).min(p3.1).min(p4.1);
        self.bbox.y_max = self.bbox.y_max.max(p1.1).max(p2.1).max(p3.1).max(p4.1);
    }

    fn push_layer(&mut self, _composite_mode: skrifa::color::CompositeMode) {}
}

struct SvgPainter<'a, 'b> {
    font: &'a FontRef<'b>,
    color_records: &'b [read_fonts::tables::cpal::ColorRecord],
    transforms: Vec<Transform>,
    clips: Vec<(Transform, OutlineGlyphOrBoundingBox<'b>)>,
    elems: Vec<Box<dyn SvgNode>>,
}

fn render_transform(tx: &Transform) -> String {
    format!(
        "matrix({} {} {} {} {} {})",
        tx.xx, tx.yx, tx.xy, tx.yy, tx.dx, tx.dy
    )
}

impl<'a, 'b> SvgPainter<'a, 'b> {
    fn new(font: &'a FontRef<'b>) -> Result<Self, anyhow::Error> {
        Ok(Self {
            font,
            color_records: font
                .cpal()?
                .color_records_array()
                .ok_or_else(|| anyhow::anyhow!("No color records found in the font"))??,
            transforms: vec![Transform::default()],
            clips: Vec::new(),
            elems: Vec::new(),
        })
    }

    fn build(&mut self) -> svg::node::element::Group {
        let mut group = svg::node::element::Group::new();
        for elem in self.elems.drain(..) {
            group.append(elem);
        }
        group
    }

    fn current_transform(&self) -> &Transform {
        self.transforms.last().unwrap()
    }
}

fn inverse_matrix_of(tx: &Transform) -> Transform {
    // Transform matrix
    //      | xx xy dx |
    // tx = | yx yy dy |
    //      |  0  0  1 |
    //
    // det(tx) = xx * yy - xy * yx
    //
    //                       | (yy)  (-xy) (xy*dy - yy*dx) |
    // inv(tx) = 1/det(tx) * | (-yx) (xx)  (yx*dx - xx*dy) |
    //                       | (0)   (0)   (det)           |
    let det = tx.xx * tx.yy - tx.xy * tx.yx;
    if det.abs() < 1e-6 {
        return Transform::default();
    }
    Transform {
        xx: tx.yy / det,
        yx: -tx.yx / det,
        xy: -tx.xy / det,
        yy: tx.xx / det,
        dx: (tx.xy * tx.dy - tx.yy * tx.dx) / det,
        dy: (tx.yx * tx.dx - tx.xx * tx.dy) / det,
    }
}

impl<'a, 'b> ColorPainter for SvgPainter<'a, 'b> {
    fn push_transform(&mut self, tx: Transform) {
        self.transforms
            .push(*self.transforms.last().unwrap() * tx);
    }

    fn pop_transform(&mut self) {
        self.transforms.pop();
    }

    fn push_clip_glyph(&mut self, glyph_id: GlyphId) {
        self.clips.push((
            *self.current_transform(),
            self.font.outline_glyphs().get(glyph_id).unwrap().into(),
        ));
    }

    fn push_clip_box(&mut self, clip_box: BoundingBox) {
        self.clips
            .push((*self.current_transform(), clip_box.into()));
    }

    fn pop_clip(&mut self) {
        self.clips.pop();
    }

    fn fill(&mut self, brush: skrifa::color::Brush<'_>) {
        let mut elem: Option<svg::node::element::Group> = None;
        for (i, (tx, clip)) in self.clips.iter().enumerate() {
            let mut pen = SvgPen::new();
            clip.draw(&mut pen);
            let mut path = svg::node::element::Path::new()
                .set("d", pen.as_ref())
                .set("transform", render_transform(tx));
            if let Some(elem_) = elem {
                let clip_id = format!("clip-{}-{}", self.elems.len(), i);
                elem = Some(
                    svg::node::element::Group::new()
                        .add(
                            svg::node::element::ClipPath::new()
                                .set("id", clip_id.clone())
                                .add(path),
                        )
                        .add(elem_.set("clip-path", format!("url(#{clip_id})"))),
                )
            } else {
                let mut g = svg::node::element::Group::new();
                match brush {
                    skrifa::color::Brush::Solid {
                        palette_index,
                        alpha,
                    } => {
                        let color = self.color_records[palette_index as usize];
                        path.assign(
                            "fill",
                            format!("rgb({}, {}, {})", color.red, color.green, color.blue,),
                        );
                        let alpha = alpha * (f32::from(color.alpha) / 255.0);
                        if alpha < 1.0 {
                            path.assign("fill-opacity", alpha);
                        }
                    }
                    skrifa::color::Brush::LinearGradient {
                        p0,
                        p1,
                        color_stops,
                        extend: _,
                    } => {
                        let grad_id = format!("grad-{}-{}", self.elems.len(), i);
                        let color_stops = color_stops
                            .iter()
                            .map(|stop| {
                                (
                                    stop.offset,
                                    (self.color_records[stop.palette_index as usize], stop.alpha),
                                )
                            });
                        let tx = *self.current_transform() * inverse_matrix_of(tx);
                        let mut lin_grad_elem = svg::node::element::LinearGradient::new()
                            .set("id", grad_id.clone())
                            .set("x1", p0.x)
                            .set("y1", p0.y)
                            .set("x2", p1.x)
                            .set("y2", p1.y)
                            .set("gradientUnits", "userSpaceOnUse")
                            .set("gradientTransform", render_transform(&tx));
                        for stop in color_stops {
                            lin_grad_elem.append(
                                svg::node::element::Stop::new()
                                    .set("offset", format!("{}%", stop.0 * 100.0))
                                    .set(
                                        "stop-color",
                                        format!(
                                            "rgb({}, {}, {})",
                                            stop.1.0.red, stop.1.0.green, stop.1.0.blue
                                        ),
                                    )
                                    .set(
                                        "stop-opacity",
                                        stop.1.1 * (f32::from(stop.1.0.alpha) / 255.0),
                                    ),
                            );
                        }
                        g.append(lin_grad_elem);
                        path = path.set("fill", format!("url(#{grad_id})"));
                    }
                    skrifa::color::Brush::RadialGradient {
                        c0,
                        r0,
                        c1,
                        r1,
                        color_stops,
                        extend: _,
                    } => {
                        let grad_id = format!("grad-{}-{}", self.elems.len(), i);
                        let color_stops = color_stops
                            .iter()
                            .map(|stop| {
                                (
                                    stop.offset,
                                    (self.color_records[stop.palette_index as usize], stop.alpha),
                                )
                            });
                        let tx = *self.current_transform() * inverse_matrix_of(tx);
                        let mut rad_grad_elem = svg::node::element::RadialGradient::new()
                            .set("id", grad_id.clone())
                            .set("cx", c0.x)
                            .set("cy", c0.y)
                            .set("r", r0)
                            .set("fx", c1.x)
                            .set("fy", c1.y)
                            .set("fr", r1)
                            .set("gradientUnits", "userSpaceOnUse")
                            .set("gradientTransform", render_transform(&tx));
                        for stop in color_stops {
                            rad_grad_elem.append(
                                svg::node::element::Stop::new()
                                    .set("offset", format!("{}%", stop.0 * 100.0))
                                    .set(
                                        "stop-color",
                                        format!(
                                            "rgb({}, {}, {})",
                                            stop.1.0.red, stop.1.0.green, stop.1.0.blue
                                        ),
                                    )
                                    .set(
                                        "stop-opacity",
                                        stop.1.1 * (f32::from(stop.1.0.alpha) / 255.0),
                                    ),
                            );
                        }
                        g.append(rad_grad_elem);
                        path = path.set("fill", format!("url(#{grad_id})"));
                    }
                    skrifa::color::Brush::SweepGradient {
                        c0,
                        start_angle: _,
                        end_angle: _,
                        color_stops,
                        extend: _,
                    } => {
                        // SVG does not support sweep gradient natively.
                        // We approximate it with a radial gradient.
                        let grad_id = format!("grad-{}-{}", self.elems.len(), i);
                        let color_stops = color_stops
                            .iter()
                            .map(|stop| {
                                (
                                    stop.offset,
                                    (self.color_records[stop.palette_index as usize], stop.alpha),
                                )
                            });
                        let tx = *self.current_transform() * inverse_matrix_of(tx);
                        let mut rad_grad_elem = svg::node::element::RadialGradient::new()
                            .set("id", grad_id.clone())
                            .set("cx", c0.x)
                            .set("cy", c0.y)
                            .set("r", 1.0)
                            .set("fx", c0.x)
                            .set("fy", c0.y)
                            .set("gradientUnits", "userSpaceOnUse")
                            .set("gradientTransform", render_transform(&tx));
                        for stop in color_stops {
                            rad_grad_elem.append(
                                svg::node::element::Stop::new()
                                    .set("offset", format!("{}%", stop.0 * 100.0))
                                    .set(
                                        "stop-color",
                                        format!(
                                            "rgb({}, {}, {})",
                                            stop.1.0.red, stop.1.0.green, stop.1.0.blue
                                        ),
                                    )
                                    .set(
                                        "stop-opacity",
                                        stop.1.1 * (f32::from(stop.1.0.alpha) / 255.0),
                                    ),
                            );
                        }
                        g.append(rad_grad_elem);
                        path = path.set("fill", format!("url(#{grad_id})"));
                    }
                }
                g.append(path);
                elem = Some(g);
            }
        }
        if let Some(elem) = elem {
            self.elems.push(Box::new(elem));
        }
    }

    fn push_layer(&mut self, _composite_mode: skrifa::color::CompositeMode) {}
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, required = true)]
    font: Vec<std::path::PathBuf>,

    #[arg(short, long, required = true)]
    output: std::path::PathBuf,

    #[arg(short, long, default_value = "24")]
    size: f32,

    codepoints: String,
}

fn get_glyph_id_for<'a>(
    font: impl AsRef<FontRef<'a>>,
    codepoints: &[char],
) -> Result<GlyphId, anyhow::Error> {
    match codepoints.len() {
        1 => font.as_ref().charmap().map(codepoints[0]),
        2 => {
            use skrifa::charmap::MapVariant::{UseDefault, Variant};
            font.as_ref()
                .charmap()
                .map_variant(codepoints[0], codepoints[1])
                .and_then(|v| match v {
                    UseDefault => font.as_ref().charmap().map(codepoints[0]),
                    Variant(glyph_id) => Some(glyph_id),
                })
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Only one or two codepoints are supported, got: {:?}",
                codepoints
            ));
        }
    }
    .ok_or_else(|| {
        anyhow::anyhow!(
            "No glyph found for codepoints '{:?}' in font {}",
            codepoints,
            font.as_ref()
                .localized_strings(StringId::new(1))
                .english_or_first()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "Unknown Font".to_string())
        )
    })
}

fn parse_codepoints(codepoints: &str) -> Result<Vec<char>, anyhow::Error> {
    codepoints
        .split(&[' ', '\t', '\n', '\r', ',', ';', ':'])
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                return None;
            }
            let codepoint = match u32::from_str_radix(s.trim_start_matches("U+"), 16) {
                Ok(cp) => cp,
                Err(e) => {
                    return Some(Err(anyhow::anyhow!("Invalid codepoint '{}': {}", s, e)));
                }
            };
            Some(
                char::from_u32(codepoint)
                    .ok_or_else(|| anyhow::anyhow!("Invalid codepoint '{}'", s)),
            )
        })
        .collect::<Result<Vec<_>, _>>()
}

struct FontContainer<'a> {
    buf: Box<[u8]>,
    font: MaybeUninit<FontRef<'a>>,
}

impl<'a> FontContainer<'a> {
    fn new(buf: Vec<u8>) -> Result<Self, anyhow::Error> {
        let mut self_ = FontContainer {
            buf: buf.into_boxed_slice(),
            font: MaybeUninit::uninit(),
        };
        self_.font.write(FontRef::new(unsafe { let raw_ptr = &raw const self_.buf; &*raw_ptr })?);
        Ok(self_)
    }
}

impl<'a> Deref for FontContainer<'a> {
    type Target = FontRef<'a>;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a> AsRef<FontRef<'a>> for FontContainer<'a> {
    fn as_ref(&self) -> &FontRef<'a> {
        unsafe { self.font.assume_init_ref() }
    }
}

fn draw_glyph_as_svg<'a, 'b>(
    font: &'a FontContainer<'b>,
    size: Size,
    codepoints: &[char],
) -> Result<svg::Document, anyhow::Error> {
    let gid = get_glyph_id_for(font, codepoints)?;
    for &draw_fn in &[try_draw_glyph_as_svg, try_draw_color_glyph_as_svg] {
        if let Some(svg) = draw_fn(font, size, gid, codepoints)? {
            return Ok(svg);
        }
    }
    Err(anyhow::anyhow!(
        "No outline or color glyph data found for {}, codepoint(s) '{:?}'",
        gid,
        codepoints
    ))
}

fn try_draw_glyph_as_svg<'a, 'b>(
    font: &'a FontContainer<'b>,
    size: Size,
    gid: GlyphId,
    _codepoints: &[char],
) -> Result<Option<svg::Document>, anyhow::Error> {
    let g = font.outline_glyphs().get(gid);
    if g.is_none() {
        return Ok(None);
    }
    try_draw_outline_glyph_as_svg(font, size, g.unwrap())
}

fn try_draw_outline_glyph_as_svg<'b>(
    font: &FontContainer<'b>,
    size: Size,
    g: OutlineGlyph<'b>,
) -> Result<Option<svg::Document>, anyhow::Error> {
    let _met = font.metrics(size, LocationRef::default());
    let mut pen = SvgPen::new();
    g.draw(size, &mut pen)?;
    let path = pen.as_ref();
    if path.is_empty() {
        return Ok(None)
    }
    let mut met_pen = ControlBoundsPen::new();
    g.draw(size, &mut met_pen)?;
    let bounds = met_pen.bounding_box().unwrap();
    let h = if bounds.y_min < 0. {
        bounds.y_max - bounds.y_min
    } else {
        size.ppem().unwrap()
    };
    let x_offset = (size.ppem().unwrap() - (bounds.x_max - bounds.x_min)) / 2.0 - bounds.x_min;
    let extent = (size.ppem().unwrap(), h);
    Ok(Some(svg::Document::new()
        .set("viewBox", (0, 0, extent.0, extent.1))
        .set("width", extent.0)
        .set("height", extent.1)
        .add(svg::node::element::Path::new().set("d", path).set(
            "transform",
            format!("translate({}, {}) scale(1, -1)", x_offset, bounds.y_max),
        ))))
}

fn try_draw_color_glyph_as_svg<'a, 'b>(
    font: &'a FontContainer<'b>,
    size: Size,
    gid: GlyphId,
    codepoints: &[char],
) -> Result<Option<svg::Document>, anyhow::Error> {
    let g = font
        .color_glyphs()
        .get(gid)
        .ok_or_else(|| {
            anyhow::anyhow!("No glyph data found for {}, codepoint(s) '{:?}'", gid, codepoints)
        })?;
    draw_color_glyph_as_svg(font, size, g, gid, codepoints).map(Some)
}

fn draw_color_glyph_as_svg<'b>(
    font: &FontContainer<'b>,
    size: Size,
    g: ColorGlyph<'b>,
    gid: GlyphId,
    codepoints: &[char],
) -> Result<svg::Document, anyhow::Error> {
    let mut painter = SvgPainter::new(font)?;
    g.paint(LocationRef::default(), &mut painter)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let bounds = g.bounding_box(LocationRef::default(), Size::unscaled()).or_else(
        || {
            let mut met_pen = CountrolBoundsPainter::new(font);
            g.paint(LocationRef::default(), &mut met_pen).ok();
            Some(met_pen.bounding_box())
        }
    ).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to compute bounding box for glyph {}, codepoint(s) '{:?}'",
            gid,
            codepoints
        )
    })?;
    let extent = (bounds.x_max - bounds.x_min, bounds.y_max - bounds.y_min);
    let ppem = size.ppem().unwrap();
    Ok(svg::Document::new()
        .set("viewBox", (0, 0, extent.0, extent.1))
        .set("width", ppem)
        .set("height", ppem * extent.1 / extent.0)
        .add(
            svg::node::element::Group::new()
                .set(
                    "transform",
                    format!("translate({}, {}) scale(1, -1)", -bounds.x_min, bounds.y_max),
                )
                .add(painter.build()),
        ))
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let fonts = args
        .font
        .iter()
        .map(|path| FontContainer::new(std::fs::read(path)?))
        .collect::<Result<Vec<_>, _>>()?;
    let size = Size::new(args.size);
    let codepoints: Vec<char> = parse_codepoints(&args.codepoints)?;

    let svg = fonts
        .iter()
        .filter_map(
            |font| match draw_glyph_as_svg(font, size, &codepoints) {
                Ok(svg) => Some(svg),
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            },
        )
        .next();
    if let Some(svg) = svg {
        return svg::save(args.output, &svg)
            .map_err(|e| anyhow::anyhow!("Failed to save SVG file: {}", e));
    }
    Err(anyhow::anyhow!(
        "No eligible font found containing a glyph for the provided codepoints"
    ))
}
