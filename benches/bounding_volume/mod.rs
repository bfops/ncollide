use ncollide::bounding_volume::{BoundingVolume, HasAABB, HasBoundingSphere};
use std::rand::IsaacRng;
use test::Bencher;
use test;
use na::Iso3;
use ncollide::bounding_volume::{AABB3, BoundingSphere3};
use ncollide::shape::{Ball3, Cuboid3, Capsule3, Cone3, Cylinder3, TriMesh3, Segment3, Triangle3,
                      Convex3};
use common::{unref, generate, generate_trimesh_around_origin};


#[path="../common/macros.rs"]
mod macros;

/*
 * Bounding volume methods.
 */
bench_method!(bench_aabb_intersects_aabb_always_true, intersects, aabb1: AABB3<f32>, aabb2: AABB3<f32>);
bench_method!(bench_bounding_sphere_intersects_bounding_sphere_always_true, intersects,
              bs1: BoundingSphere3<f32>, bs2: BoundingSphere3<f32>);

bench_method!(bench_aabb_contains_aabb, contains, aabb1: AABB3<f32>, aabb2: AABB3<f32>);
bench_method!(bench_bounding_sphere_contains_bounding_sphere, contains,
              bs1: BoundingSphere3<f32>, bs2: BoundingSphere3<f32>);

bench_method!(bench_aabb_merged_aabb, merged, aabb1: AABB3<f32>, aabb2: AABB3<f32>);
bench_method!(bench_bounding_sphere_merged_bounding_sphere, merged,
              bs1: BoundingSphere3<f32>, bs2: BoundingSphere3<f32>);

bench_method!(bench_aabb_loosened_aabb, loosened, aabb1: AABB3<f32>, margin: f32);
bench_method!(bench_bounding_sphere_loosened_bounding_sphere, loosened,
              bs1: BoundingSphere3<f32>, margin: f32);

/*
 * Bounding volume construction.
 */
bench_method!(bench_cuboid_aabb, aabb, c: Cuboid3<f32>, m: Iso3<f32>);
bench_method!(bench_cuboid_bounding_sphere, bounding_sphere, c: Cuboid3<f32>, m: Iso3<f32>);

bench_method!(bench_ball_aabb, aabb, b: Ball3<f32>, m: Iso3<f32>);
bench_method!(bench_ball_bounding_sphere, bounding_sphere, b: Ball3<f32>, m: Iso3<f32>);

bench_method!(bench_capsule_aabb, aabb, c: Capsule3<f32>, m: Iso3<f32>);
bench_method!(bench_capsule_bounding_sphere, bounding_sphere, c: Capsule3<f32>, m: Iso3<f32>);

bench_method!(bench_cone_aabb, aabb, c: Cone3<f32>, m: Iso3<f32>);
bench_method!(bench_cone_bounding_sphere, bounding_sphere, c: Cone3<f32>, m: Iso3<f32>);

bench_method!(bench_cylinder_aabb, aabb, c: Cylinder3<f32>, m: Iso3<f32>);
bench_method!(bench_cylinder_bounding_sphere, bounding_sphere, c: Cylinder3<f32>, m: Iso3<f32>);

bench_method!(bench_segment_aabb, aabb, c: Segment3<f32>, m: Iso3<f32>);
bench_method!(bench_segment_bounding_sphere, bounding_sphere, c: Segment3<f32>, m: Iso3<f32>);

bench_method!(bench_triangle_aabb, aabb, c: Triangle3<f32>, m: Iso3<f32>);
bench_method!(bench_triangle_bounding_sphere, bounding_sphere, c: Triangle3<f32>, m: Iso3<f32>);

bench_method!(bench_convex_aabb, aabb, c: Convex3<f32>, m: Iso3<f32>);
bench_method!(bench_convex_bounding_sphere, bounding_sphere, c: Convex3<f32>, m: Iso3<f32>);

bench_method_gen!(bench_mesh_aabb, aabb,
                  mesh: TriMesh3<f32> = generate_trimesh_around_origin,
                  m: Iso3<f32> = generate);
bench_method_gen!(bench_mesh_bounding_sphere, bounding_sphere,
                  mesh: TriMesh3<f32> = generate_trimesh_around_origin,
                  m: Iso3<f32> = generate);
