use std::io::{Read, Seek, SeekFrom, Error as IOError};
use std::iter;

use byteorder::{ReadBytesExt, BE};

#[derive(Clone, Debug)]
pub enum SceneGeomFormat {
    Unknown,
    Imf,
    Hmf,
    Hxf,
    Hxf2,
    Vu1,
    Vu1Paged,
    Ixf,
    Nxf,
}

impl SceneGeomFormat {
    fn from_u32(val: u32) -> SceneGeomFormat {
        match val {
            1 => SceneGeomFormat::Imf,
            2 => SceneGeomFormat::Hmf,
            3 => SceneGeomFormat::Hxf,
            4 => SceneGeomFormat::Hxf2,
            6 => SceneGeomFormat::Vu1,
            7 => SceneGeomFormat::Vu1Paged,
            8 => SceneGeomFormat::Ixf,
            9 => SceneGeomFormat::Nxf,
            _ => SceneGeomFormat::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScenePlacementData {
    Static(SceneGeomFormat),
    StaticInst(SceneGeomFormat),
    Animated,
    AnimatedInst,
    Ground(SceneGeomFormat),
    GroundVU1(SceneGeomFormat),
    Point(u32),
    DirLight {
        sub_type: u32,
        r: f32,
        g: f32,
        b: f32,
    },
    AmbientLight {
        sub_type: u32,
        r: f32,
        g: f32,
        b: f32,
    },
    Camera {
        sub_type: u32,
        interest_x: f32,
        interest_y: f32,
        interest_z: f32,
        field_of_view: f32,
    },
    Path_,
    AnimWithPath,
    AnimWithoutPath,
    BoundingBox {
        sub_type: u32,
        min: (f32, f32, f32, f32),
        max: (f32, f32, f32, f32),
    },
    WorldSprite,
    PointList,
    Sky(SceneGeomFormat),
    Bezier {
        sub_type: u32,
        length: f32,
        degree: u32,
        closed: u32,
        param_type: u32,
        nb_knots: u32,
        nb_control_points: u32,
        control_points: u32,
        knots: u32,
        curve_points: u32,
        true_length: f32,
        pad: [u32; 5],
    },
    ColCylinder {
        sub_type: u32,
        min: (f32, f32, f32, f32),
        max: (f32, f32, f32, f32),
    },
    CoverList,
    CombatPath,
    Unknown(u32, u32, Vec<u8>),
}

impl ScenePlacementData {
    fn from_bytes(main_type: u32, sub_type: u32, data: Vec<u8>) -> Result<ScenePlacementData, IOError> {
        let mut read = &data[..];
        match main_type {
            0 => Ok(ScenePlacementData::Static(SceneGeomFormat::from_u32(sub_type))),
            1 => Ok(ScenePlacementData::StaticInst(SceneGeomFormat::from_u32(sub_type))),
            4 => Ok(ScenePlacementData::Ground(SceneGeomFormat::from_u32(sub_type))),
            5 => Ok(ScenePlacementData::GroundVU1(SceneGeomFormat::from_u32(sub_type))),
            6 => Ok(ScenePlacementData::Point(sub_type)),
            7 => Ok(ScenePlacementData::DirLight {
                sub_type: sub_type,
                r: read.read_f32::<BE>()?,
                g: read.read_f32::<BE>()?,
                b: read.read_f32::<BE>()?,
            }),
            8 => Ok(ScenePlacementData::AmbientLight {
                sub_type: sub_type,
                r: read.read_f32::<BE>()?,
                g: read.read_f32::<BE>()?,
                b: read.read_f32::<BE>()?,
            }),
            9 => Ok(ScenePlacementData::Camera {
                sub_type: sub_type,
                interest_x: read.read_f32::<BE>()?,
                interest_y: read.read_f32::<BE>()?,
                interest_z: read.read_f32::<BE>()?,
                field_of_view: read.read_f32::<BE>()?,
            }),
            13 => Ok(ScenePlacementData::BoundingBox {
                sub_type: sub_type,
                min: (read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?),
                max: (read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?),
            }),
            20 => Ok(ScenePlacementData::Sky(SceneGeomFormat::from_u32(sub_type))),
            22 => Ok(ScenePlacementData::Bezier {
                sub_type: sub_type,
                length: read.read_f32::<BE>()?,
                degree: read.read_u32::<BE>()?,
                closed: read.read_u32::<BE>()?,
                param_type: read.read_u32::<BE>()?,
                nb_knots: read.read_u32::<BE>()?,
                nb_control_points: read.read_u32::<BE>()?,
                control_points: read.read_u32::<BE>()?,
                knots: read.read_u32::<BE>()?,
                curve_points: read.read_u32::<BE>()?,
                true_length: read.read_f32::<BE>()?,
                pad: [
                    read.read_u32::<BE>()?,
                    read.read_u32::<BE>()?,
                    read.read_u32::<BE>()?,
                    read.read_u32::<BE>()?,
                    read.read_u32::<BE>()?,
                ],
            }),
            25 => Ok(ScenePlacementData::ColCylinder {
                sub_type: sub_type,
                min: (read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?),
                max: (read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?, read.read_f32::<BE>()?),
            }),
            _ => Ok(ScenePlacementData::Unknown(main_type, sub_type, data)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScenePlacement {
    pub model_name: String,
    pub geom_name: String,
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    pub w_pos: f32,
    pub x_rot: f32,
    pub y_rot: f32,
    pub z_rot: f32,
    pub w_rot: f32,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub w_scale: f32,
    pub data: ScenePlacementData,
}

impl ScenePlacement {
    fn from_read<R>(mut read: R) -> Result<ScenePlacement, IOError>
        where R: Read + Seek
    {
        let main_type = read.read_u32::<BE>()?;
        let sub_type = read.read_u32::<BE>()?;

        let mut model_name_bytes = [0; 0x20];
        read.read_exact(&mut model_name_bytes)?;
        let model_name_len = model_name_bytes
            .iter()
            .position(|x| *x == 0)
            .unwrap_or(0x20);
        let model_name = String::from_utf8(model_name_bytes[0..model_name_len].to_owned()).unwrap();

        let mut geom_name_bytes = [0; 0x20];
        read.read_exact(&mut geom_name_bytes)?;
        let geom_name_len = geom_name_bytes
            .iter()
            .position(|x| *x == 0)
            .unwrap_or(0x20);
        let geom_name = String::from_utf8(geom_name_bytes[0..geom_name_len].to_owned()).unwrap();

        let x_pos = read.read_f32::<BE>()?;
        let y_pos = read.read_f32::<BE>()?;
        let z_pos = read.read_f32::<BE>()?;
        let w_pos = read.read_f32::<BE>()?;
        let x_rot = read.read_f32::<BE>()?;
        let y_rot = read.read_f32::<BE>()?;
        let z_rot = read.read_f32::<BE>()?;
        let w_rot = read.read_f32::<BE>()?;
        let x_scale = read.read_f32::<BE>()?;
        let y_scale = read.read_f32::<BE>()?;
        let z_scale = read.read_f32::<BE>()?;
        let w_scale = read.read_f32::<BE>()?;

        let data_len = read.read_u32::<BE>()?;
        let mut data_vec = iter::repeat(0)
            .take(data_len as usize)
            .collect::<Vec<u8>>();
        read.read_exact(&mut data_vec)?;

        let data = ScenePlacementData::from_bytes(main_type, sub_type, data_vec)?;

        Ok(
            ScenePlacement {
                model_name: model_name,
                geom_name: geom_name,
                x_pos: x_pos,
                y_pos: y_pos,
                z_pos: z_pos,
                w_pos: w_pos,
                x_rot: x_rot,
                y_rot: y_rot,
                z_rot: z_rot,
                w_rot: w_rot,
                x_scale: x_scale,
                y_scale: y_scale,
                z_scale: z_scale,
                w_scale: w_scale,
                data: data,
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct SceneClump {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub placements: Vec<ScenePlacement>,
}

impl SceneClump {
    fn from_read<R>(mut read: R) -> Result<SceneClump, IOError>
        where R: Read + Seek
    {
        let num_placements = read.read_u16::<BE>()?;
        let _pad = read.read_u16::<BE>()?;
        let min_x = read.read_f32::<BE>()?;
        let max_x = read.read_f32::<BE>()?;
        let min_z = read.read_f32::<BE>()?;
        let max_z = read.read_f32::<BE>()?;

        let mut placements = Vec::new();
        for _ in 0..num_placements {
            let placement = ScenePlacement::from_read(&mut read)?;
            placements.push(placement);
        }

        Ok (
            SceneClump {
                min_x: min_x,
                max_x: max_x,
                min_z: min_z,
                max_z: max_z,
                placements: placements,
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct SceneTemplate {
    pub header: u32,
    pub format: u32,
    pub version: f32,
    pub name: String,
    pub x_cut_size: f32,
    pub z_cut_size: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub clumps: Vec<SceneClump>,
}

impl SceneTemplate {
    pub fn from_read<R>(mut read: R) -> Result<SceneTemplate, IOError>
        where R: Read + Seek
    {
        let header = read.read_u32::<BE>()?;
        let format = read.read_u32::<BE>()?;
        let version = read.read_f32::<BE>()?;

        let mut name_bytes = [0; 0x20];
        read.read_exact(&mut name_bytes)?;
        let name_len = name_bytes
            .iter()
            .position(|x| *x == 0)
            .unwrap_or(0x20);
        let name = String::from_utf8(name_bytes[0..name_len].to_owned()).unwrap();

        let x_cut_size = read.read_f32::<BE>()?;
        let z_cut_size = read.read_f32::<BE>()?;
        let min_x = read.read_f32::<BE>()?;
        let max_x = read.read_f32::<BE>()?;
        let min_z = read.read_f32::<BE>()?;
        let max_z = read.read_f32::<BE>()?;

        let num_clumps = read.read_u16::<BE>()?;
        let _pad = read.read_u16::<BE>()?;
        let mut clumps = Vec::new();
        for _ in 0..num_clumps {
            let offset = read.read_u32::<BE>()?;
            let save = read.seek(SeekFrom::Current(0))?;
            read.seek(SeekFrom::Start(offset as u64))?;
            let clump = SceneClump::from_read(&mut read)?;
            clumps.push(clump);
            read.seek(SeekFrom::Start(save as u64))?;
        }

        Ok(
            SceneTemplate {
                header: header,
                format: format,
                version: version,
                name: name,
                x_cut_size: x_cut_size,
                z_cut_size: z_cut_size,
                min_x: min_x,
                max_x: max_x,
                min_z: min_z,
                max_z: max_z,
                clumps: clumps,
            }
        )
    }
}