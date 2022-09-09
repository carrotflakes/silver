use std::{fs, io};

use crate::{materials::Basic as Material, shapes::Basic as Shape, vec3::Vec3};

pub fn from_yaml(file: &str) -> Result<Vec<(Shape, Material)>, String> {
    let file = fs::File::open(file).unwrap();
    let reader = io::BufReader::new(file);
    let scene: map::Scene = serde_yaml::from_reader(reader).map_err(|e| e.to_string())?;

    Ok(scene
        .objects
        .into_iter()
        .map(|object| object.into())
        .collect())
}

mod map {
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub struct Scene {
        pub objects: Vec<Object>,
    }

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub struct Object {
        pub shape: Shape,
        pub material: Material,
    }

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Shape {
        Sphere { center: [f64; 3], radius: f64 },
    }

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Material {
        Lambertian { color: [f64; 3] },
    }
}

impl Into<(Shape, Material)> for map::Object {
    fn into(self) -> (Shape, Material) {
        (self.shape.into(), self.material.into())
    }
}

impl Into<Shape> for map::Shape {
    fn into(self) -> Shape {
        match self {
            map::Shape::Sphere { center, radius } => {
                Shape::Sphere(crate::shapes::Sphere::new(Vec3::new(center), radius))
            }
        }
    }
}

impl Into<Material> for map::Material {
    fn into(self) -> Material {
        match self {
            map::Material::Lambertian { color } => {
                Material::Lambertian(crate::materials::Lambertian::new(Vec3::new(color)))
            }
        }
    }
}

#[test]
fn test() {
    use map::*;

    println!(
        "{}",
        serde_yaml::to_string(&Scene {
            objects: vec![Object {
                shape: Shape::Sphere {
                    center: [0.0, 10.0, 20.0],
                    radius: 100.0,
                },
                material: Material::Lambertian {
                    color: [0.7, 0.0, 0.0],
                },
            }],
        })
        .unwrap()
    );

    println!(
        "{:?}",
        serde_yaml::from_str::<Scene>(include_str!("../../scene.yml"))
    )
}
