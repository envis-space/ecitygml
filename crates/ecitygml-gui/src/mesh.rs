use ecitygml_core::model::common::LevelOfDetail;
use ecitygml_core::store::ImplicitGeometryPlacement;
use egml::model::common::Triangulate;
use egml::model::geometry::AbstractGeometryKind;
use egml::model::geometry::primitives::TriangulatedSurface;
use nalgebra::{Matrix4, Point3};
use std::collections::HashMap;
use three_d::{CpuMesh, InnerSpace, Mat4, Vec3};

/// Level of detail preference used whenever multiple representations of the same object are
/// available and only one can be shown at a time.
const LOD_PRIORITY: [LevelOfDetail; 4] = [
    LevelOfDetail::Two,
    LevelOfDetail::Three,
    LevelOfDetail::One,
    LevelOfDetail::Zero,
];

/// Converts an already-triangulated surface (see
/// `CityModelGraphicsStore::best_triangulated_surface`) into a GPU-ready mesh.
///
/// Implicit geometries are drawn separately — as GPU instances in `Scene::from_store` (see
/// `prototype_to_cpu_mesh`/`placement_instance_transform`), and not at all in the single-object
/// highlight overlay in `Scene::build_highlight` — so they're never a source here.
pub fn triangulated_surface_to_cpu_mesh(
    ts: &TriangulatedSurface,
    center: &Point3<f64>,
) -> Option<CpuMesh> {
    let triangles = ts.triangles();
    if triangles.is_empty() {
        return None;
    }

    let mut positions: Vec<Vec3> = Vec::with_capacity(triangles.len() * 3);
    let mut normals: Vec<Vec3> = Vec::with_capacity(triangles.len() * 3);

    for tri in &triangles {
        let a = to_vec3(tri.a(), center);
        let b = to_vec3(tri.b(), center);
        let c = to_vec3(tri.c(), center);

        let normal = (b - a).cross(c - a).normalize();
        positions.extend_from_slice(&[a, b, c]);
        normals.extend_from_slice(&[normal, normal, normal]);
    }

    let indices: Vec<u32> = (0u32..(positions.len() as u32)).collect();

    Some(CpuMesh {
        positions: three_d::Positions::F32(positions),
        normals: Some(normals),
        indices: three_d::Indices::U32(indices),
        ..Default::default()
    })
}

fn to_vec3(pos: &egml::model::geometry::DirectPosition, center: &Point3<f64>) -> Vec3 {
    Vec3::new(
        (pos.x() - center.x) as f32,
        (pos.y() - center.y) as f32,
        (pos.z() - center.z) as f32,
    )
}

/// Triangulates a prototype geometry in its own local coordinates — unlike
/// `triangulate_resolved_implicit_geometry`, this does *not* place it in world space, since with
/// instancing that placement is the per-instance GPU transform (see
/// `placement_instance_transform`), not something baked into the shared mesh once up front.
pub fn prototype_to_cpu_mesh(geometry: &AbstractGeometryKind) -> Option<CpuMesh> {
    let triangulation = match geometry.triangulate() {
        Ok(triangulation) => triangulation,
        Err(e) => {
            tracing::warn!("prototype geometry triangulation failed: {e}");
            return None;
        }
    };
    for skipped in triangulation.skipped() {
        tracing::warn!("prototype geometry member skipped during triangulation: {skipped}");
    }
    // No world placement here, so no center offset to apply either.
    triangulated_surface_to_cpu_mesh(triangulation.surface(), &Point3::origin())
}

/// Reduces `placements` to at most one per owner, keeping each owner's best-available LOD —
/// mirrors the LOD selection `CityModelGraphicsStore::best_triangulated_surface` applies to
/// non-implicit geometry, so an object with implicit geometry at more than one LOD doesn't get
/// instanced at every LOD at once.
pub fn best_placement_per_owner<'a>(
    placements: &[(
        &'a egml::model::base::Id,
        LevelOfDetail,
        &'a ImplicitGeometryPlacement,
    )],
) -> Vec<(&'a egml::model::base::Id, &'a ImplicitGeometryPlacement)> {
    let mut best: HashMap<&egml::model::base::Id, (LevelOfDetail, &ImplicitGeometryPlacement)> =
        HashMap::new();

    for &(feature_id, lod, placement) in placements {
        best.entry(feature_id)
            .and_modify(|(current_lod, current)| {
                if lod_rank(lod) < lod_rank(*current_lod) {
                    *current_lod = lod;
                    *current = placement;
                }
            })
            .or_insert((lod, placement));
    }

    best.into_iter()
        .map(|(feature_id, (_, placement))| (feature_id, placement))
        .collect()
}

fn lod_rank(lod: LevelOfDetail) -> usize {
    LOD_PRIORITY
        .iter()
        .position(|&candidate| candidate == lod)
        .unwrap_or(LOD_PRIORITY.len())
}

/// Builds the per-instance world transform for one placement of a prototype geometry: the
/// `transformationMatrix` (rotation/scale/shear, applied first) composed with a translation to
/// `reference_point` (see `ImplicitGeometryPlacement::world_transform`), as a matrix for the
/// GPU to apply per instance instead of a one-off baked into vertex positions. `center` is
/// subtracted from the translation for the same f32-precision reason `to_vec3` subtracts it from
/// vertices — prepending a `-center` translation shifts only the translation column, leaving the
/// rotation/scale part untouched.
pub fn placement_instance_transform(
    placement: &ImplicitGeometryPlacement,
    center: &Point3<f64>,
) -> Mat4 {
    let center_offset = Matrix4::new_translation(&(-center.coords));
    let combined = center_offset * placement.instance_transform().matrix();
    let m = combined.as_slice();

    Mat4::new(
        m[0] as f32,
        m[1] as f32,
        m[2] as f32,
        m[3] as f32,
        m[4] as f32,
        m[5] as f32,
        m[6] as f32,
        m[7] as f32,
        m[8] as f32,
        m[9] as f32,
        m[10] as f32,
        m[11] as f32,
        m[12] as f32,
        m[13] as f32,
        m[14] as f32,
        m[15] as f32,
    )
}
