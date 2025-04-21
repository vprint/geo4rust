use gdal::vector::{ Feature, Geometry };

pub fn get_extent_as_geometry(feature: &Feature) -> Option<Geometry> {
    let geom_option = feature.geometry();

    match geom_option {
        Some(geom) => {
            let env = geom.envelope();
            let w = env.MinX;
            let s = env.MinY;
            let e = env.MaxX;
            let n = env.MaxY;

            match Geometry::bbox(w, s, e, n) {
                Ok(geometry) => Some(geometry),
                Err(_) => None,
            }
        }
        None => None,
    }
}

/// Determines if two features are adjacent using touches operator.
///
/// # Arguments
///
/// * `a` - The first feature.
/// * `b` - The feature to compare.
///
/// # Returns
///
/// * `boolean` True if the geometries touch each other, false otherwise.
pub fn is_adjacent(a: &Feature, b: &Feature) -> bool {
    let a_geom_option = a.geometry();
    let b_geom_option = b.geometry();

    match (a_geom_option, b_geom_option) {
        (Some(a_geom), Some(b_geom)) => a_geom.intersects(&b_geom),
        _ => false,
    }
}
