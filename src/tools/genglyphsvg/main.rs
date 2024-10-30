use std::ops::Deref;

use clap::Parser;
use skrifa::instance::Size;
use skrifa::metrics::BoundingBox;
use skrifa::outline::pen::{OutlinePen, SvgPen};
use skrifa::prelude::LocationRef;
use skrifa::string::StringId;
use skrifa::{FontRef, GlyphId, MetadataProvider};

struct MetricsExtractorPen(BoundingBox);

impl MetricsExtractorPen {
    fn new() -> Self {
        Self(BoundingBox {
            x_min: f32::INFINITY,
            x_max: f32::NEG_INFINITY,
            y_min: f32::INFINITY,
            y_max: f32::NEG_INFINITY,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        self.0
    }
}

impl OutlinePen for MetricsExtractorPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.x_min = self.0.x_min.min(x);
        self.0.x_max = self.0.x_max.max(x);
        self.0.y_min = self.0.y_min.min(y);
        self.0.y_max = self.0.y_max.max(y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.x_min = self.0.x_min.min(x);
        self.0.x_max = self.0.x_max.max(x);
        self.0.y_min = self.0.y_min.min(y);
        self.0.y_max = self.0.y_max.max(y);
    }

    fn quad_to(&mut self, _cx0: f32, _cy0: f32, x: f32, y: f32) {
        self.0.x_min = self.0.x_min.min(x);
        self.0.x_max = self.0.x_max.max(x);
        self.0.y_min = self.0.y_min.min(y);
        self.0.y_max = self.0.y_max.max(y);
    }

    fn curve_to(&mut self, _cx0: f32, _cy0: f32, _cx1: f32, _cy1: f32, x: f32, y: f32) {
        self.0.x_min = self.0.x_min.min(x);
        self.0.x_max = self.0.x_max.max(x);
        self.0.y_min = self.0.y_min.min(y);
        self.0.y_max = self.0.y_max.max(y);
    }

    fn close(&mut self) {
    }
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
                .and_then(|v| {match v {
                    UseDefault => font.as_ref().charmap().map(codepoints[0]),
                    Variant(glyph_id) => Some(glyph_id),
                }})
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
    font: Option<FontRef<'a>>,
}

impl<'a> FontContainer<'a> {
    fn new(buf: Vec<u8>) -> Result<Box<Self>, anyhow::Error> {
        let mut self_ = Box::new(Self {
            buf: buf.into_boxed_slice(),
            font: None,
        });
        self_.font = Some(FontRef::new(unsafe { &*(&raw const *self_.buf) })?);
        Ok(self_)
    }
}

impl<'a> Deref for FontContainer<'a> {
    type Target = FontRef<'a>;

    fn deref(&self) -> &Self::Target {
        self.font.as_ref().unwrap()
    }
}

impl<'a> AsRef<FontRef<'a>> for FontContainer<'a> {
    fn as_ref(&self) -> &FontRef<'a> {
        self.font.as_ref().unwrap()
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let fonts = args
        .font
        .iter()
        .map(|path| FontContainer::new(std::fs::read(&path)?))
        .collect::<Result<Vec<_>, _>>()?;
    let size = Size::new(args.size);
    let codepoints: Vec<char> = parse_codepoints(&args.codepoints)?;

    for font in fonts {
        let gid = match get_glyph_id_for(font.as_ref(), &codepoints) {
            Ok(gid) => gid,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        //let met = font.metrics(size, LocationRef::default());
        let g = font.outline_glyphs().get(gid).ok_or_else(|| {
            anyhow::anyhow!("No outline glyph found for codepoint(s) '{:?}'", codepoints)
        })?;
        let met = font.metrics(size, LocationRef::default());
        let mut pen = SvgPen::new();
        g.draw(size, &mut pen)?;
        let mut met_pen = MetricsExtractorPen::new();
        g.draw(size, &mut met_pen)?;
        let bounds = met_pen.bounding_box();
        let h = if bounds.y_min < 0. {
            bounds.y_max - bounds.y_min
        } else {
            size.ppem().unwrap()
        };
        let x_offset = (size.ppem().unwrap() - (bounds.x_max - bounds.x_min)) / 2.0 - bounds.x_min;
        let extent = (size.ppem().unwrap(), h);
        let document = svg::Document::new()
            .set("viewBox", (0, 0, extent.0, extent.1))
            .set("width", extent.0)
            .set("height", extent.1)
            .add(
                svg::node::element::Path::new()
                .set("d", pen.as_ref())
                .set("transform", format!("translate({}, {}) scale(1, -1)", x_offset, bounds.y_max))
            );
        svg::save(args.output, &document)
            .map_err(|e| anyhow::anyhow!("Failed to save SVG file: {}", e))?;
        return Ok(());
    }
    Err(anyhow::anyhow!(
        "No eligible font found containing a glyph for the provided codepoints"
    ))
}
