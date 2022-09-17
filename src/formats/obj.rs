use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn load(
    obj_path: &str,
) -> (
    Vec<([([f32; 3], [f32; 2], [f32; 3]); 3], i32)>,
    Vec<Material>,
) {
    let file = File::open(obj_path).unwrap();
    let reader = BufReader::new(file);

    let mut vs: Vec<[f32; 3]> = vec![];
    let mut vts: Vec<[f32; 2]> = vec![];
    let mut vns: Vec<[f32; 3]> = vec![];
    let mut fs: Vec<([[i32; 3]; 3], i32)> = vec![];
    let mut ms = vec![];

    let mut mtl = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("mtllib") => {
                let name = tokens.next().unwrap();
                let mtl_path = Path::new(obj_path).parent().unwrap().join(name);
                ms.extend(load_mtl(mtl_path));
            }
            Some("v") => {
                vs.push(
                    tokens
                        .take(3)
                        .map(|t| t.parse::<f32>())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                );
            }
            Some("vt") => {
                vts.push(
                    tokens
                        .take(2)
                        .map(|t| t.parse::<f32>())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                );
            }
            Some("vn") => {
                vns.push(
                    tokens
                        .take(3)
                        .map(|t| t.parse::<f32>())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                );
            }
            Some("f") => {
                let vs = tokens
                    .map(|t| {
                        t.split("/")
                            .map(|n| n.parse::<i32>().unwrap_or(0))
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                match vs.len() {
                    3 => fs.push(([vs[0], vs[1], vs[2]], mtl)),
                    4 => {
                        fs.push(([vs[0], vs[1], vs[2]], mtl));
                        fs.push(([vs[2], vs[3], vs[0]], mtl));
                    }
                    _ => panic!("multi-poly"),
                }
            }
            Some("usemtl") => {
                let mtl_name = tokens.next().unwrap();
                mtl = ms.iter().position(|m| m.name == mtl_name).unwrap_or(0) as i32;
            }
            Some(_) => {}
            None => {}
        }
    }

    (
        fs.iter()
            .map(|f| {
                (
                    [
                        (
                            vs[f.0[0][0] as usize - 1],
                            vts[f.0[0][1] as usize - 1],
                            vns[f.0[0][2] as usize - 1],
                        ),
                        (
                            vs[f.0[1][0] as usize - 1],
                            vts[f.0[1][1] as usize - 1],
                            vns[f.0[1][2] as usize - 1],
                        ),
                        (
                            vs[f.0[2][0] as usize - 1],
                            vts[f.0[2][1] as usize - 1],
                            vns[f.0[2][2] as usize - 1],
                        ),
                    ],
                    f.1,
                )
            })
            .collect(),
        ms,
    )
}

pub struct Material {
    name: String,
    ns: f64,
    ka: [f64; 3], // ambient color
    kd: [f64; 3], // diffuse color
    ks: [f64; 3], // specular color
    ke: [f64; 3],
    map_kd: String,
    ni: f64, // shineness
    d: f64,  // opacity
    illum: bool,
}

pub fn load_mtl(mtl_path: impl AsRef<Path>) -> Vec<Material> {
    let file = File::open(mtl_path).unwrap();
    let reader = BufReader::new(file);

    let mut ms = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("newmtl") => ms.push(Material {
                name: tokens.next().unwrap().to_string(),
                ns: 0.0,
                ka: [0.0; 3],
                kd: [0.0; 3],
                ks: [0.0; 3],
                ke: [0.0; 3],
                map_kd: String::new(),
                ni: 0.0,
                d: 0.0,
                illum: false,
            }),
            Some("Ns") => {
                ms.last_mut().unwrap().ns = tokens.next().unwrap().parse().unwrap();
            }
            Some("Ka") => {
                ms.last_mut().unwrap().ka = tokens
                    .take(3)
                    .map(|t| t.parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
            Some("Kd") => {
                ms.last_mut().unwrap().kd = tokens
                    .take(3)
                    .map(|t| t.parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
            Some("Ks") => {
                ms.last_mut().unwrap().ks = tokens
                    .take(3)
                    .map(|t| t.parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
            Some("Ke") => {
                ms.last_mut().unwrap().ke = tokens
                    .take(3)
                    .map(|t| t.parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
            Some("map_Kd") => {
                ms.last_mut().unwrap().map_kd = tokens.next().unwrap().to_string();
            }
            Some("Ni") => {
                ms.last_mut().unwrap().ni = tokens.next().unwrap().parse().unwrap();
            }
            Some("d") => {
                ms.last_mut().unwrap().d = tokens.next().unwrap().parse().unwrap();
            }
            Some("illum") => {
                ms.last_mut().unwrap().illum = tokens.next().unwrap().parse::<i32>().unwrap() == 1;
            }
            _ => {}
        }
    }
    ms
}
