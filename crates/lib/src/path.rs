use std::fmt::{self, Display, Formatter};

use crate::vec::Vec2;

pub struct Path {
    pub segments: Vec<PathSegment>,
    pub fill: String,
    pub stroke: String,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<path d=\"")?;
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                write!(f, " ")?;
            }
            segment.fmt(f)?;
        }
        write!(f, "\" fill=\"{}\" stroke=\"{}\" />", self.fill, self.stroke)
    }
}

/// The first bool indicates whether the coordinates are relative
pub enum PathSegment {
    Close,
    Move(Vec2),
    Line(Vec2),
    QuadraticBezier(Vec2, Vec2),
    CubicBezier(Vec2, Vec2, Vec2),
    Arc {
        radius: Vec2,
        rotation: f64,
        large_arc: bool,
        sweep: bool,
        position: Vec2,
    },
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Close => write!(f, "Z"),
            Self::Move(pos) => {
                write!(f, "M ")?;
                pos.fmt(f)
            }
            Self::Line(pos) => {
                write!(f, "L ")?;
                pos.fmt(f)
            }
            Self::QuadraticBezier(b, c) => {
                write!(f, "Q ")?;
                b.fmt(f)?;
                write!(f, " ")?;
                c.fmt(f)
            }
            Self::CubicBezier(b, c, d) => {
                write!(f, "C ")?;
                b.fmt(f)?;
                write!(f, " ")?;
                c.fmt(f)?;
                write!(f, " ")?;
                d.fmt(f)
            }
            Self::Arc {
                radius,
                rotation,
                large_arc,
                sweep,
                position,
            } => {
                write!(f, "A ")?;
                radius.fmt(f)?;
                write!(f, " ")?;
                match f.precision() {
                    Some(precision) => write!(
                        f,
                        "{}",
                        format!("{rotation:.*}", precision)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                    ),
                    None => write!(f, "{rotation}"),
                }?;
                write!(f, " {} {} ", *large_arc as u8, *sweep as u8)?;
                position.fmt(f)
            }
        }
    }
}

pub struct QuadraticBezier(pub Vec2, pub Vec2, pub Vec2);
pub struct CubicBezier(pub Vec2, pub Vec2, pub Vec2, pub Vec2);

impl QuadraticBezier {
    pub fn as_cubic(&self) -> CubicBezier {
        CubicBezier(
            self.0,
            self.0 + (self.1 - self.0) * (2. / 3.),
            self.2 + (self.1 - self.2) * (2. / 3.),
            self.2,
        )
    }
}

impl CubicBezier {
    pub fn halves(&self) -> (Self, Self) {
        let CubicBezier(a, b, c, d) = self;

        let e = a.halfway_point(b);
        let f = b.halfway_point(c);
        let g = c.halfway_point(d);

        let h = e.halfway_point(&f);
        let j = f.halfway_point(&g);

        let k = h.halfway_point(&j);

        (CubicBezier(*a, e, h, k), CubicBezier(k, j, g, *d))
    }
}
