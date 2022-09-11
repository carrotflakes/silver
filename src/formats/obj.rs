use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load(file: &str) -> Vec<[([f32; 3], [f32; 2], [f32; 3]); 3]> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    let mut vs: Vec<[f32; 3]> = vec![];
    let mut vts: Vec<[f32; 2]> = vec![];
    let mut vns: Vec<[f32; 3]> = vec![];
    let mut fs: Vec<[[i32; 3]; 3]> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("mtllib") => {
                // let name = tokens.next().unwrap();
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
                    3 => fs.push([vs[0], vs[1], vs[2]]),
                    4 => {
                        fs.push([vs[0], vs[1], vs[2]]);
                        fs.push([vs[2], vs[3], vs[0]]);
                    }
                    _ => panic!("multi-poly"),
                }
            }
            Some(_) => {}
            None => {}
        }
    }

    fs.iter()
        .map(|f| {
            [
                (
                    vs[f[0][0] as usize - 1],
                    vts[f[0][1] as usize - 1],
                    vns[f[0][2] as usize - 1],
                ),
                (
                    vs[f[1][0] as usize - 1],
                    vts[f[1][1] as usize - 1],
                    vns[f[1][2] as usize - 1],
                ),
                (
                    vs[f[2][0] as usize - 1],
                    vts[f[2][1] as usize - 1],
                    vns[f[2][2] as usize - 1],
                ),
            ]
        })
        .collect()
}
