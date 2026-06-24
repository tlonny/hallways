use glam::Vec3;
use parry3d::math::{Isometry, Vector};

use crate::level::cache::CacheEntry;
use crate::level::Cache;

use super::Kinematics;

pub struct SweepHit {
    pub time: f32,
    pub normal: Vec3,
    pub point: Vec3,
}

impl Kinematics {
    pub fn sweep(&self, cache: &mut Cache, velocity: Vec3, max_toi: f32) -> Option<SweepHit> {
        let level_url = self.level_url.as_ref().unwrap();
        let Some(CacheEntry::Ready(level)) = cache.get(level_url) else {
            return None;
        };
        let collider = self.stance.collider();
        let shape_pos = Isometry::translation(self.position.x, self.position.y, self.position.z);
        let shape_vel = Vector::new(velocity.x, velocity.y, velocity.z);

        let mut best_hit: Option<SweepHit> = level
            .sweep(&shape_pos, &shape_vel, &collider, max_toi)
            .map(|hit| {
                return SweepHit {
                    time: hit.time_of_impact,
                    normal: Vec3::new(hit.normal2.x, hit.normal2.y, hit.normal2.z),
                    point: Vec3::new(hit.witness2.x, hit.witness2.y, hit.witness2.z),
                };
            });

        for src_portal in level.portals() {
            let name = &src_portal.name;
            let portal_hit = src_portal
                .sweep(&shape_pos, &shape_vel, &collider, max_toi)
                .map(|r| SweepHit {
                    time: r.time_of_impact,
                    normal: Vec3::new(r.normal2.x, r.normal2.y, r.normal2.z),
                    point: Vec3::new(r.witness2.x, r.witness2.y, r.witness2.z),
                });
            let Some(portal_hit) = portal_hit else {
                continue;
            };

            let link = src_portal.link(cache);
            let Some(link) = link else {
                match &best_hit {
                    Some(best) if best.time <= portal_hit.time => {}
                    _ => best_hit = Some(portal_hit),
                }
                continue;
            };
            let target = src_portal.target.as_ref().unwrap();

            let Some(CacheEntry::Ready(dst_level)) = cache.get(&target.url) else {
                panic!("linked level not ready")
            };

            let Some(dst_portal) = dst_level.portals().get(link.portal_ix) else {
                continue;
            };
            let Some(dst_target) = dst_portal.target.as_ref() else {
                match &best_hit {
                    Some(best) if best.time <= portal_hit.time => {}
                    _ => best_hit = Some(portal_hit),
                }
                continue;
            };
            if dst_target.url.as_str() != level_url.as_str() || &dst_target.name != name {
                match &best_hit {
                    Some(best) if best.time <= portal_hit.time => {}
                    _ => best_hit = Some(portal_hit),
                }
                continue;
            }
            let transformed_pos = link.transform_position(self.position);
            let transformed_vel = link.transform_velocity(velocity);

            let shape_pos =
                Isometry::translation(transformed_pos.x, transformed_pos.y, transformed_pos.z);
            let shape_vel = Vector::new(transformed_vel.x, transformed_vel.y, transformed_vel.z);
            let result_hit = dst_level.sweep(&shape_pos, &shape_vel, &collider, max_toi);
            let Some(result_hit) = result_hit else {
                continue;
            };

            let dst_link = dst_portal.link(cache).unwrap();
            let dst_normal = Vec3::new(
                result_hit.normal2.x,
                result_hit.normal2.y,
                result_hit.normal2.z,
            );
            let dst_point = Vec3::new(
                result_hit.witness2.x,
                result_hit.witness2.y,
                result_hit.witness2.z,
            );
            let through_portal_hit = SweepHit {
                time: result_hit.time_of_impact,
                normal: dst_link.transform_velocity(dst_normal),
                point: dst_link.transform_position(dst_point),
            };

            match &best_hit {
                Some(best) if best.time <= through_portal_hit.time => {}
                _ => best_hit = Some(through_portal_hit),
            }
        }

        return best_hit;
    }
}
