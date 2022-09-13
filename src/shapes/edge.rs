use crate::{
    bbox::BBox,
    ray::Ray,
    shapes::{HitRec, Shape},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Edge {
    vertexes: [Vec3; 2],
    radiuses: [f64; 2],
}

impl Edge {
    pub fn new(vertexes: [Vec3; 2], radiuses: [f64; 2]) -> Self {
        Self { vertexes, radiuses }
    }
}

impl Shape for Edge {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRec> {
        if let Some([a, b]) = calc_ray_cylinder(
            &ray.origin,
            &ray.direction,
            &self.vertexes[0],
            &self.vertexes[1],
            self.radiuses[0],
            self.radiuses[1],
        ) {
            if t0 < a.0 && a.0 < t1 && (0.0..1.0).contains(&a.1) {
                return Some(HitRec {
                    time: a.0,
                    location: ray.at(a.0),
                    normal: (a.2).normalize(),
                    uv: [0.0, 0.0],
                    front: true,
                });
            }
            if t0 < b.0 && b.0 < t1 && (0.0..1.0).contains(&b.1) {
                return Some(HitRec {
                    time: b.0,
                    location: ray.at(b.0),
                    normal: (-b.2).normalize(),
                    uv: [0.0, 0.0],
                    front: false,
                });
            }
        }
        None
    }

    fn bbox(&self) -> BBox {
        BBox::from_min_max(
            Vec3::new([
                (self.vertexes[0].x() - self.radiuses[0])
                    .min(self.vertexes[1].x() - self.radiuses[1]),
                (self.vertexes[0].y() - self.radiuses[0])
                    .min(self.vertexes[1].y() - self.radiuses[1]),
                (self.vertexes[0].z() - self.radiuses[0])
                    .min(self.vertexes[1].z() - self.radiuses[1]),
            ]),
            Vec3::new([
                (self.vertexes[0].x() + self.radiuses[0])
                    .max(self.vertexes[1].x() + self.radiuses[1]),
                (self.vertexes[0].y() + self.radiuses[0])
                    .max(self.vertexes[1].y() + self.radiuses[1]),
                (self.vertexes[0].z() + self.radiuses[0])
                    .max(self.vertexes[1].z() + self.radiuses[1]),
            ]),
        )
    }
}

// ref http://marupeke296.com/COL_3D_No25_RayToSilinder.html
fn calc_ray_cylinder(
    // レイの始点
    l: &Vec3,
    // レイの方向ベクトル
    v: &Vec3,
    // 円柱軸の1点
    p1: &Vec3,
    // 円柱軸のもう1点
    p2: &Vec3,
    // 円柱の半径
    r1: f64,
    r2: f64,
) -> Option<[(f64, f64, Vec3); 2]> {
    let p = *p1 - *l;
    let p2 = *p2 - *l;
    let s = p2 - p;

    // 各種内積値
    let dvv = v.dot(v);
    let dsv = s.dot(v);
    let dpv = p.dot(v);
    let dss = s.dot(&s);
    let dps = p.dot(&s);
    let dpp = p.dot(&p);

    if dss == 0.0 {
        return None; // 円柱が定義されない
    }

    let idss = dss.recip();
    // let A = dvv - dsv * dsv / dss;
    // let B = dpv - dps * dsv / dss;
    // let C = dpp - dps * dps / dss - r1 * r2;
    let f_a = dvv - dsv * dsv * idss - (dsv * idss * (r2 - r1)).powi(2);
    let f_b = dpv - dps * dsv * idss + (dsv * idss * (r2 - r1) * (r1 - dps * idss * (r2 - r1)));
    let f_c = dpp - dps * dps * idss - (r1 - dps * idss * (r2 - r1)).powi(2);

    if f_a == 0.0 {
        return None;
    }

    let f_d = f_b * f_b - f_a * f_c;
    if f_d < 0.0 {
        return None; // レイが円柱と衝突していない
    }
    let f_d = f_d.sqrt();

    let a1 = (f_b - f_d) / f_a;
    let a2 = (f_b + f_d) / f_a;
    let b1 = a1 * dsv * idss - dps * idss;
    let b2 = a2 * dsv * idss - dps * idss;

    let x = r1 * (r2 - r1) * idss;
    Some([
        (a1, b1, a1 * *v - (p + (b1 + x) * s)), // TODO: fix normal!!!
        (a2, b2, a2 * *v - (p + (b2 + x) * s)),
    ])
}
