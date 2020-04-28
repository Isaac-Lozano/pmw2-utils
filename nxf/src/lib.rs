use std::io::{Read, Seek, SeekFrom, Error as IOError};

use byteorder::{ReadBytesExt, BE};

trait ReadFileExt: Seek {
    type Err;
    fn read_at_offset<T, F>(&mut self, offset: u64, f: F) -> Result<T, Self::Err>
        where F: Fn(&mut Self) -> Result<T, Self::Err>;
    fn read_string(&mut self) -> Result<String, Self::Err>;
}

impl<R> ReadFileExt for R
    where R: Read + Seek,
{
    type Err = IOError;
    fn read_at_offset<T, F>(&mut self, offset: u64, f: F) -> Result<T, Self::Err>
        where F: Fn(&mut Self) -> Result<T, Self::Err>,
    {
        let saved_offset = self.seek(SeekFrom::Current(0))?;
        self.seek(SeekFrom::Start(offset))?;
        let result = f(self)?;
        self.seek(SeekFrom::Start(saved_offset))?;
        Ok(result)
    }

    fn read_string(&mut self) -> Result<String, Self::Err> {
        let mut buffer = Vec::new();
        let mut bytes = self.bytes();
        loop {
            if let Some(byte_res) = bytes.next() {
                let byte = byte_res?;
                if byte == 0 {
                    break;
                } else {
                    buffer.push(byte);
                }
            } else {
                break;
            }
        }
        Ok(String::from_utf8(buffer).unwrap())
    }
}

#[derive(Clone, Debug)]
pub struct NxfMaterial {
    pub tex_pmi: u32,
    pub ref_pmi: u32,
    pub tex_name: String,
    pub ref_map: u32,
    pub ref_r: u8,
    pub ref_g: u8,
    pub ref_b: u8,
    pub ref_a: u8,
    pub flags: u32,
    pub alpha_mode: u32,
    pub env_map_alpha_mode: u32,
}

impl NxfMaterial {
    pub fn from_read<R>(mut read: R) -> Result<NxfMaterial, IOError>
        where R: Read + Seek
    {
        let tex_pmi = read.read_u32::<BE>()?;
        let ref_pmi = read.read_u32::<BE>()?;

        let tex_name_offset = read.read_u32::<BE>()?;
        let tex_name = read.read_at_offset(tex_name_offset as u64, |read| {
            Ok(read.read_string()?)
        })?;

        let ref_map = read.read_u32::<BE>()?;

        let ref_r = read.read_u8()?;
        let ref_g = read.read_u8()?;
        let ref_b = read.read_u8()?;
        let ref_a = read.read_u8()?;

        let flags = read.read_u32::<BE>()?;
        let alpha_mode = read.read_u32::<BE>()?;
        let env_map_alpha_mode = read.read_u32::<BE>()?;

        let _pad1 = read.read_u32::<BE>()?;
        let _pad2 = read.read_u32::<BE>()?;

        Ok(NxfMaterial {
            tex_pmi: tex_pmi,
            ref_pmi: ref_pmi,
            tex_name: tex_name,
            ref_map: ref_map,
            ref_r: ref_r,
            ref_g: ref_g,
            ref_b: ref_b,
            ref_a: ref_a,
            flags: flags,
            alpha_mode: alpha_mode,
            env_map_alpha_mode: env_map_alpha_mode,
        })
    }

    pub fn list_from_read<R>(mut read: R, mut offset: u64) -> Result<Vec<NxfMaterial>, IOError>
        where R: Read + Seek
    {
        let save = read.seek(SeekFrom::Current(0))?;
        let mut materials = Vec::new();
        while offset != 0 {
            read.seek(SeekFrom::Start(offset))?;
            materials.push(NxfMaterial::from_read(&mut read)?);
            offset = read.read_u32::<BE>()? as u64;
        }
        read.seek(SeekFrom::Start(save))?;
        Ok(materials)
    }
}

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn from_read<R>(mut read: R) -> Result<Vec3, IOError>
        where R: Read
    {
        let x = read.read_f32::<BE>()?;
        let y = read.read_f32::<BE>()?;
        let z = read.read_f32::<BE>()?;

        Ok(Vec3 {
            x: x,
            y: y,
            z: z,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_read<R>(mut read: R) -> Result<Color, IOError>
        where R: Read
    {
        let r = read.read_u8()?;
        let g = read.read_u8()?;
        let b = read.read_u8()?;
        let a = read.read_u8()?;

        Ok(Color {
            r: r,
            g: g,
            b: b,
            a: a,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl Uv {
    pub fn from_read<R>(mut read: R) -> Result<Uv, IOError>
        where R: Read
    {
        let u = read.read_f32::<BE>()?;
        let v = read.read_f32::<BE>()?;

        Ok(Uv {
            u: u,
            v: v,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfArray {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
    pub c_x: f32,
    pub c_y: f32,
    pub c_z: f32,
    pub radius: f32,
    pub max_verts: u32,
    pub max_normals: u32,
    pub max_cols: u32,
    pub max_uvs: u32,
    pub verts: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub colors: Vec<Color>,
    pub uvs: Vec<Uv>,
    pub flags: u32,
}

impl NxfArray {
    pub fn from_read<R>(mut read: R) -> Result<NxfArray, IOError>
        where R: Read + Seek
    {
        let min_x = read.read_f32::<BE>()?;
        let min_y = read.read_f32::<BE>()?;
        let min_z = read.read_f32::<BE>()?;

        let num_uvs = read.read_u32::<BE>()?;

        let max_x = read.read_f32::<BE>()?;
        let max_y = read.read_f32::<BE>()?;
        let max_z = read.read_f32::<BE>()?;

        let num_normals = read.read_u32::<BE>()?;

        let c_x = read.read_f32::<BE>()?;
        let c_y = read.read_f32::<BE>()?;
        let c_z = read.read_f32::<BE>()?;
        let radius = read.read_f32::<BE>()?;

        let num_verts = read.read_u32::<BE>()?;
        let num_cols = read.read_u32::<BE>()?;
        let max_verts = read.read_u32::<BE>()?;
        let max_normals = read.read_u32::<BE>()?;
        let max_cols = read.read_u32::<BE>()?;
        let max_uvs = read.read_u32::<BE>()?;

        let verts_offset = read.read_u32::<BE>()?;
        let verts = if verts_offset != 0 {
            read.read_at_offset(verts_offset as u64, |mut read| {
                let mut verts = Vec::new();
                for _ in 0..num_verts {
                    let vert = Vec3::from_read(&mut read)?;
                    verts.push(vert);
                }
                Ok(verts)
            })?
        } else {
            Vec::new()
        };

        let normals_offset = read.read_u32::<BE>()?;
        let normals = if normals_offset != 0 {
            read.read_at_offset(normals_offset as u64, |mut read| {
                let mut normals = Vec::new();
                for _ in 0..num_normals {
                    let normal = Vec3::from_read(&mut read)?;
                    normals.push(normal);
                }
                Ok(normals)
            })?
        } else {
            Vec::new()
        };

        let colors_offset = read.read_u32::<BE>()?;
        let colors = if colors_offset != 0 {
            read.read_at_offset(colors_offset as u64, |mut read| {
                let mut colors = Vec::new();
                for _ in 0..num_cols {
                    let color = Color::from_read(&mut read)?;
                    colors.push(color);
                }
                Ok(colors)
            })?
        } else {
            Vec::new()
        };

        let uvs_offset = read.read_u32::<BE>()?;
        let uvs = if uvs_offset != 0 {
            read.read_at_offset(uvs_offset as u64, |mut read| {
                let mut uvs = Vec::new();
                for _ in 0..num_uvs {
                    let uv = Uv::from_read(&mut read)?;
                    uvs.push(uv);
                }
                Ok(uvs)
            })?
        } else {
            Vec::new()
        };

        let flags = read.read_u32::<BE>()?;

        let _pad1 = read.read_u32::<BE>()?;
        let _pad2 = read.read_u32::<BE>()?;

        Ok(NxfArray {
            min_x: min_x,
            min_y: min_y,
            min_z: min_z,
            max_x: max_x,
            max_y: max_y,
            max_z: max_z,
            c_x: c_x,
            c_y: c_y,
            c_z: c_z,
            radius: radius,
            max_verts: max_verts,
            max_normals: max_normals,
            max_cols: max_cols,
            max_uvs: max_uvs,
            verts: verts,
            normals: normals,
            colors: colors,
            uvs: uvs,
            flags: flags,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfColLitTri {
    pub v0: u16,
    pub n0: u16,
    pub c0: u16,
    pub v1: u16,
    pub n1: u16,
    pub c1: u16,
    pub v2: u16,
    pub n2: u16,
    pub c2: u16,
}

impl NxfColLitTri {
    pub fn from_read<R>(mut read: R) -> Result<NxfColLitTri, IOError>
        where R: Read
    {
        Ok(NxfColLitTri {
            v0: read.read_u16::<BE>()?,
            n0: read.read_u16::<BE>()?,
            c0: read.read_u16::<BE>()?,
            v1: read.read_u16::<BE>()?,
            n1: read.read_u16::<BE>()?,
            c1: read.read_u16::<BE>()?,
            v2: read.read_u16::<BE>()?,
            n2: read.read_u16::<BE>()?,
            c2: read.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfTexLitTri {
    pub v0: u16,
    pub n0: u16,
    pub c0: u16,
    pub uv0: u16,
    pub v1: u16,
    pub n1: u16,
    pub c1: u16,
    pub uv1: u16,
    pub v2: u16,
    pub n2: u16,
    pub c2: u16,
    pub uv2: u16,
}

impl NxfTexLitTri {
    pub fn from_read<R>(mut read: R) -> Result<NxfTexLitTri, IOError>
        where R: Read
    {
        Ok(NxfTexLitTri {
            v0: read.read_u16::<BE>()?,
            n0: read.read_u16::<BE>()?,
            c0: read.read_u16::<BE>()?,
            uv0: read.read_u16::<BE>()?,
            v1: read.read_u16::<BE>()?,
            n1: read.read_u16::<BE>()?,
            c1: read.read_u16::<BE>()?,
            uv1: read.read_u16::<BE>()?,
            v2: read.read_u16::<BE>()?,
            n2: read.read_u16::<BE>()?,
            c2: read.read_u16::<BE>()?,
            uv2: read.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfTexUnlitTri {
    pub v0: u16,
    pub c0: u16,
    pub uv0: u16,
    pub v1: u16,
    pub c1: u16,
    pub uv1: u16,
    pub v2: u16,
    pub c2: u16,
    pub uv2: u16,
}

impl NxfTexUnlitTri {
    pub fn from_read<R>(mut read: R) -> Result<NxfTexUnlitTri, IOError>
        where R: Read
    {
        Ok(NxfTexUnlitTri {
            v0: read.read_u16::<BE>()?,
            c0: read.read_u16::<BE>()?,
            uv0: read.read_u16::<BE>()?,
            v1: read.read_u16::<BE>()?,
            c1: read.read_u16::<BE>()?,
            uv1: read.read_u16::<BE>()?,
            v2: read.read_u16::<BE>()?,
            c2: read.read_u16::<BE>()?,
            uv2: read.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfColUnlitTri {
    pub v0: u16,
    pub c0: u16,
    pub v1: u16,
    pub c1: u16,
    pub v2: u16,
    pub c2: u16,
}

impl NxfColUnlitTri {
    pub fn from_read<R>(mut read: R) -> Result<NxfColUnlitTri, IOError>
        where R: Read
    {
        Ok(NxfColUnlitTri {
            v0: read.read_u16::<BE>()?,
            c0: read.read_u16::<BE>()?,
            v1: read.read_u16::<BE>()?,
            c1: read.read_u16::<BE>()?,
            v2: read.read_u16::<BE>()?,
            c2: read.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfTexLitEnvTri {
    pub v0: u16,
    pub n0: u16,
    pub c0: u16,
    pub uv0: u16,
    pub m0: u16,
    pub v1: u16,
    pub n1: u16,
    pub c1: u16,
    pub uv1: u16,
    pub m1: u16,
    pub v2: u16,
    pub n2: u16,
    pub c2: u16,
    pub uv2: u16,
    pub m2: u16,
}

impl NxfTexLitEnvTri {
    pub fn from_read<R>(mut read: R) -> Result<NxfTexLitEnvTri, IOError>
        where R: Read
    {
        Ok(NxfTexLitEnvTri {
            v0: read.read_u16::<BE>()?,
            n0: read.read_u16::<BE>()?,
            c0: read.read_u16::<BE>()?,
            uv0: read.read_u16::<BE>()?,
            m0: read.read_u16::<BE>()?,
            v1: read.read_u16::<BE>()?,
            n1: read.read_u16::<BE>()?,
            c1: read.read_u16::<BE>()?,
            uv1: read.read_u16::<BE>()?,
            m1: read.read_u16::<BE>()?,
            v2: read.read_u16::<BE>()?,
            n2: read.read_u16::<BE>()?,
            c2: read.read_u16::<BE>()?,
            uv2: read.read_u16::<BE>()?,
            m2: read.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NxfColLitEnvTri {
    pub v0: u16,
    pub n0: u16,
    pub c0: u16,
    pub m0: u16,
    pub v1: u16,
    pub n1: u16,
    pub c1: u16,
    pub m1: u16,
    pub v2: u16,
    pub n2: u16,
    pub c2: u16,
    pub m2: u16,
}

impl NxfColLitEnvTri {
    pub fn from_read<R>(mut read: R) -> Result<NxfColLitEnvTri, IOError>
        where R: Read
    {
        Ok(NxfColLitEnvTri {
            v0: read.read_u16::<BE>()?,
            n0: read.read_u16::<BE>()?,
            c0: read.read_u16::<BE>()?,
            m0: read.read_u16::<BE>()?,
            v1: read.read_u16::<BE>()?,
            n1: read.read_u16::<BE>()?,
            c1: read.read_u16::<BE>()?,
            m1: read.read_u16::<BE>()?,
            v2: read.read_u16::<BE>()?,
            n2: read.read_u16::<BE>()?,
            c2: read.read_u16::<BE>()?,
            m2: read.read_u16::<BE>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum NxfFaces {
    ColLitTri(Vec<NxfColLitTri>),
    TexLitTri(Vec<NxfTexLitTri>),
    TexUnlitTri(Vec<NxfTexUnlitTri>),
    ColUnlitTri(Vec<NxfColUnlitTri>),
    TexLitEnvTri(Vec<NxfTexLitEnvTri>),
    ColLitEnvTri(Vec<NxfColLitEnvTri>),
}

impl NxfFaces {
    pub fn from_read<R>(mut read: R, facelist_type: u8, num: u32) -> Result<NxfFaces, IOError>
        where R: Read
    {
        match facelist_type {
            6 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(NxfColLitTri::from_read(&mut read)?);
                }
                Ok(NxfFaces::ColLitTri(faces))
            }
            8 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(NxfTexLitTri::from_read(&mut read)?);
                }
                Ok(NxfFaces::TexLitTri(faces))
            }
            10 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(NxfTexUnlitTri::from_read(&mut read)?);
                }
                Ok(NxfFaces::TexUnlitTri(faces))
            }
            11 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(NxfColUnlitTri::from_read(&mut read)?);
                }
                Ok(NxfFaces::ColUnlitTri(faces))
            }
            20 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(NxfTexLitEnvTri::from_read(&mut read)?);
                }
                Ok(NxfFaces::TexLitEnvTri(faces))
            }
            21 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(NxfColLitEnvTri::from_read(&mut read)?);
                }
                Ok(NxfFaces::ColLitEnvTri(faces))
            }
            _ => panic!("Bad face type"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            NxfFaces::ColLitTri(faces) => faces.len(),
            NxfFaces::TexLitTri(faces) => faces.len(),
            NxfFaces::TexUnlitTri(faces) => faces.len(),
            NxfFaces::ColUnlitTri(faces) => faces.len(),
            NxfFaces::TexLitEnvTri(faces) => faces.len(),
            NxfFaces::ColLitEnvTri(faces) => faces.len(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NxfFacelist {
    pub flags: u16,
    pub attribs: u8,
    pub material: NxfMaterial,
    pub faces: NxfFaces,
    next_facelist: u64, // XXX: needed (for now) so I can read a list of these
    pub display_list: u32,
    pub display_list_size: u32,
}

impl NxfFacelist {
    pub fn from_read<R>(mut read: R) -> Result<NxfFacelist, IOError>
        where R: Read + Seek
    {
        let flags = read.read_u16::<BE>()?;
        let facelist_type = read.read_u8()?;
        let attribs = read.read_u8()?;
        let _pad = read.read_u32::<BE>()?;

        let material_offset = read.read_u32::<BE>()? as u64;
        let material = read.read_at_offset(material_offset, |mut read| {
            NxfMaterial::from_read(&mut read)
        })?;

        let num_faces = read.read_u32::<BE>()?;
        let faces_offset = read.read_u32::<BE>()? as u64;
        let faces = read.read_at_offset(faces_offset, |mut read| {
            NxfFaces::from_read(&mut read, facelist_type, num_faces)
        })?;

        let next_facelist = read.read_u32::<BE>()? as u64;

        let display_list = read.read_u32::<BE>()?;
        let display_list_size = read.read_u32::<BE>()?;

        Ok(NxfFacelist {
            flags: flags,
            attribs: attribs,
            material: material,
            faces: faces,
            next_facelist: next_facelist,
            display_list: display_list,
            display_list_size: display_list_size,
        })
    }

    pub fn list_from_read<R>(mut read: R, mut offset: u64) -> Result<Vec<NxfFacelist>, IOError>
        where R: Read + Seek
    {
        let save = read.seek(SeekFrom::Current(0))?;
        let mut facelists = Vec::new();
        while offset != 0 {
            read.seek(SeekFrom::Start(offset))?;
            let facelist = NxfFacelist::from_read(&mut read)?;
            offset = facelist.next_facelist;
            facelists.push(facelist);
        }
        read.seek(SeekFrom::Start(save))?;
        Ok(facelists)
    }
}

#[derive(Clone, Debug)]
pub struct NxfMatrixPalette;

#[derive(Clone, Debug)]
pub struct NxfFacelistSet {
    pub flags: u32,
    pub facelists: Vec<NxfFacelist>,
    pub mat_palette: Option<NxfMatrixPalette>,
}

impl NxfFacelistSet {
    pub fn from_read<R>(mut read: R) -> Result<NxfFacelistSet, IOError>
        where R: Read + Seek
    {
        let flags = read.read_u32::<BE>()?;
        let _pad = read.read_u32::<BE>()?;

        let _num_lists = read.read_u32::<BE>()?;
        let first_facelist = read.read_u32::<BE>()? as u64;
        let facelists = NxfFacelist::list_from_read(&mut read, first_facelist)?;

        // TODO: read mat palettes
        let _mat_palette_offset = read.read_u32::<BE>()?;

        Ok(NxfFacelistSet {
            flags: flags,
            facelists: facelists,
            mat_palette: None,
        })
    }

    pub fn list_from_read<R>(mut read: R, mut offset: u64) -> Result<Vec<NxfFacelistSet>, IOError>
        where R: Read + Seek
    {
        let save = read.seek(SeekFrom::Current(0))?;
        let mut facelist_sets = Vec::new();
        while offset != 0 {
            read.seek(SeekFrom::Start(offset))?;
            facelist_sets.push(NxfFacelistSet::from_read(&mut read)?);
            offset = read.read_u32::<BE>()? as u64;
        }
        read.seek(SeekFrom::Start(save))?;
        Ok(facelist_sets)
    }
}

#[derive(Clone, Debug)]
pub struct NxfObjGeom {
    pub id: [u8; 4],
    pub endian: u32,
    pub version: f32,
    pub flags: u32,
    pub alpha_mode: u32,
    pub env_map_alpha_mode: u32,
    pub strings: Vec<String>,
    pub materials: Vec<NxfMaterial>,
    pub arrays: NxfArray,
    pub facelist_sets: Vec<NxfFacelistSet>,
    pub display_list: u32,
    pub display_list_size: u32,
}

impl NxfObjGeom {
    pub fn from_read<R>(mut read: R) -> Result<NxfObjGeom, IOError>
        where R: Read + Seek
    {
        let mut id = [0; 4];
        read.read_exact(&mut id)?;
        let endian = read.read_u32::<BE>()?;
        let version = read.read_f32::<BE>()?;
        let flags = read.read_u32::<BE>()?;
        let alpha_mode = read.read_u32::<BE>()?;
        let env_map_alpha_mode = read.read_u32::<BE>()?;

        let num_strings = read.read_u16::<BE>()?;
        let _pad = read.read_u16::<BE>()?;
        let strings_offset = read.read_u32::<BE>()?;
        let strings = read.read_at_offset(strings_offset as u64, |read| {
            let mut strings = Vec::new();
            for _ in 0..num_strings {
                let string_offset = read.read_u32::<BE>()?;
                let s = read.read_at_offset(string_offset as u64, |read| {
                    Ok(read.read_string()?)
                })?;
                strings.push(s);
            }
            Ok(strings)
        })?;

        let material_offset = read.read_u32::<BE>()?;
        let materials = NxfMaterial::list_from_read(&mut read, material_offset as u64)?;

        let arrays_offset = read.read_u32::<BE>()?;
        let arrays = read.read_at_offset(arrays_offset as u64, |read| {
            NxfArray::from_read(read)
        })?;

        let first_facelist_set = read.read_u32::<BE>()?;
        let facelist_sets = NxfFacelistSet::list_from_read(&mut read, first_facelist_set as u64)?;

        let display_list = read.read_u32::<BE>()?;
        let display_list_size = read.read_u32::<BE>()?;
        let _expanded = read.read_u32::<BE>()?; // TODO: read more geoms
        let _pad1 = read.read_u32::<BE>()?;
        let _pad2 = read.read_u32::<BE>()?;
        let _pad3 = read.read_u32::<BE>()?;

        Ok(NxfObjGeom {
            id: id,
            endian: endian,
            version: version,
            flags: flags,
            alpha_mode: alpha_mode,
            env_map_alpha_mode: env_map_alpha_mode,
            strings: strings,
            materials: materials,
            arrays: arrays,
            facelist_sets: facelist_sets,
            display_list: display_list,
            display_list_size: display_list_size,
        })
    }
}