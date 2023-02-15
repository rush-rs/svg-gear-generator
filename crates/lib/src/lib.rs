use path::{Path, PathSegment, QuadraticBezier};
use vec::Vec2;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod path;
pub mod vec;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy)]
pub struct GearSettings {
    pub radius: f64,
    pub inner_radius: f64,
    pub center: Vec2,
    pub rotation: f64,
    pub groove_count: usize,
    /// Value between 0 and 1 where 0 is half the radius and 1 is the full radius
    pub groove_depth: f64,
    /// Value between -1 and 1, bigger values cause smaller spikes
    pub width_proportion: f64,
    pub cutoff: usize,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl GearSettings {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        radius: f64,
        inner_radius: f64,
        center: Vec2,
        rotation: f64,
        groove_count: usize,
        groove_depth: f64,
        width_proportion: f64,
        cutoff: usize,
    ) -> Self {
        Self {
            radius,
            inner_radius,
            center,
            rotation,
            groove_count,
            groove_depth,
            width_proportion,
            cutoff,
        }
    }
}

impl Default for GearSettings {
    fn default() -> Self {
        Self {
            radius: 90.,
            inner_radius: 45.,
            center: vec2!(100.),
            rotation: 216.,
            groove_count: 10,
            groove_depth: 0.4,
            width_proportion: 0.2,
            cutoff: 4,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn gen_gear_string(
    settings: GearSettings,
    fill_color: &str,
    stroke_color: &str,
    float_precision: usize,
) -> String {
    let path = gen_gear(settings, fill_color, stroke_color);
    format!("{path:.*}", float_precision)
}

pub fn gen_gear(settings: GearSettings, fill_color: &str, stroke_color: &str) -> Path {
    // calculate all corner points

    let base_vec = settings.center - vec2!(0., settings.radius);
    let angle_delta = 360. / (settings.groove_count * 2) as f64;
    let proportion_diff = angle_delta * settings.width_proportion;
    let angle_delta_spike = angle_delta - proportion_diff;
    let angle_delta_groove = angle_delta + proportion_diff;

    let mut corner_points: Vec<(Vec2, Vec2)> = vec![];
    let mut angle = angle_delta_groove / 2.;

    let start_point = base_vec.rotated(angle + settings.rotation, settings.center);
    angle += angle_delta_spike;

    while angle < 360. {
        let point1 = base_vec.rotated(angle + settings.rotation, settings.center);
        angle += angle_delta_groove;
        let point2 = base_vec.rotated(angle + settings.rotation, settings.center);
        angle += angle_delta_spike;
        corner_points.push((point1, point2));
    }

    // now use the calculated points to create the path

    let mut segments = vec![PathSegment::Move(start_point)];
    let mut did_cutoff = false;
    for (index, (point1, point2)) in corner_points.iter().enumerate() {
        // arc
        segments.push(PathSegment::Arc {
            radius: vec2!(settings.radius),
            rotation: 0.,
            large_arc: false,
            sweep: true,
            position: *point1,
        });

        let curve_point = calc_curve_point(point1, point2, &settings);
        if index + 1 == settings.cutoff && settings.cutoff < settings.groove_count {
            // half groove at end
            let curve = QuadraticBezier(*point1, curve_point, *point2)
                .as_cubic()
                .halves()
                .0;
            segments.push(PathSegment::CubicBezier(curve.1, curve.2, curve.3));
            did_cutoff = true;
            break;
        }
        // groove
        segments.push(PathSegment::QuadraticBezier(curve_point, *point2));
    }

    if did_cutoff {
        let cutoff_angle = settings.cutoff as f64 * (angle_delta_spike + angle_delta_groove);
        let inner_circle_end = (settings.center - vec2!(0., settings.inner_radius))
            .rotated(settings.rotation, settings.center);
        let inner_circle_start = inner_circle_end.rotated(cutoff_angle, settings.center);

        // line to circle start
        segments.push(PathSegment::Line(inner_circle_start));
        // inner circle
        segments.push(PathSegment::Arc {
            radius: vec2!(settings.inner_radius),
            rotation: 0.,
            large_arc: cutoff_angle > 180.,
            sweep: false,
            position: inner_circle_end,
        });

        let (point1, point2) = corner_points[corner_points.len() - 1];
        let curve = QuadraticBezier(
            point1,
            calc_curve_point(&point1, &point2, &settings),
            point2,
        )
        .as_cubic()
        .halves()
        .1;
        // line from circle end
        segments.push(PathSegment::Line(curve.0));
        // half groove at beginning
        segments.push(PathSegment::CubicBezier(curve.1, curve.2, curve.3));
        // close path
        segments.push(PathSegment::Close);
    } else {
        // close path
        segments.push(PathSegment::Close);
        // inner circle
        let top = settings.center - vec2!(0., settings.inner_radius);
        let bottom = settings.center + vec2!(0., settings.inner_radius);
        segments.push(PathSegment::Move(top));
        segments.push(PathSegment::Arc {
            radius: vec2!(1.),
            rotation: 0.,
            large_arc: false,
            sweep: false,
            position: bottom,
        });
        segments.push(PathSegment::Arc {
            radius: vec2!(1.),
            rotation: 0.,
            large_arc: false,
            sweep: false,
            position: top,
        });
        // close path
        segments.push(PathSegment::Close);
    }

    Path {
        segments,
        fill: fill_color.to_string(),
        stroke: stroke_color.to_string(),
    }
}

fn calc_curve_point(point1: &Vec2, point2: &Vec2, settings: &GearSettings) -> Vec2 {
    let dir_vec = point1.halfway_point(point2) - settings.center;
    settings.center + (1. - settings.groove_depth) * dir_vec
}
