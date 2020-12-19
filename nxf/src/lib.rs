use std::io::{Read, Seek, SeekFrom, Result as IoResult};
use std::rc::Rc;

use binfile::de::{Deserializer, Deserialize};
use binfile::shared_pointer::{
    SharedPointerDeserializeState,
    SharedPointer,
    SharedPointed,
};
use binfile::single_pointer::{
    SinglePointer,
};
use binfile::types::{
    TerminatedString,
    SizedList,
};

#[derive(Clone, Debug)]
pub struct NxfMaterial {
    pub tex_pmi: u32,
    pub ref_pmi: u32,
    pub tex_name: Rc<String>,
    pub ref_map: u32,
    pub ref_r: u8,
    pub ref_g: u8,
    pub ref_b: u8,
    pub ref_a: u8,
    pub flags: u32,
    pub alpha_mode: u32,
    pub env_map_alpha_mode: u32,
}

impl<'a> Deserialize<'a> for NxfMaterial {
    type Output = Self;
    type State = &'a mut SharedPointerDeserializeState<String>;

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        let tex_pmi = de.read_u32()?;
        let ref_pmi = de.read_u32()?;

        let tex_name_shared = de.deserialize::<SharedPointer<TerminatedString>>((state, ()))?;
        let tex_name = tex_name_shared;

        let ref_map = de.read_u32()?;

        let ref_r = de.read_u8()?;
        let ref_g = de.read_u8()?;
        let ref_b = de.read_u8()?;
        let ref_a = de.read_u8()?;

        let flags = de.read_u32()?;
        let alpha_mode = de.read_u32()?;
        let env_map_alpha_mode = de.read_u32()?;

        let _pad1 = de.read_u32()?;
        let _pad2 = de.read_u32()?;

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
}

struct NxfMaterialListDeserializer;

impl<'a> Deserialize<'a> for NxfMaterialListDeserializer {
    type Output = Vec<Rc<NxfMaterial>>;
    type State = (&'a mut SharedPointerDeserializeState<NxfMaterial>, &'a mut SharedPointerDeserializeState<String>);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self::Output>
    where
        R: Read + Seek,
    {
        let mut materials = Vec::new();
        let mut data_addr = de.read_u32()? as u64;
        let save_addr = de.inner().seek(SeekFrom::Current(0))?;

        while data_addr != 0 {
            de.inner().seek(SeekFrom::Start(data_addr))?;
            let material = de.deserialize::<SharedPointer<NxfMaterial>>(state)?;
            data_addr = de.read_u32()? as u64;
            materials.push(material);
        }

        de.inner().seek(SeekFrom::Start(save_addr))?;

        Ok(materials)
    }
}

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<'a> Deserialize<'a> for Vec3 {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        let x = de.read_f32()?;
        let y = de.read_f32()?;
        let z = de.read_f32()?;

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

impl<'a> Deserialize<'a> for Color {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        let r = de.read_u8()?;
        let g = de.read_u8()?;
        let b = de.read_u8()?;
        let a = de.read_u8()?;

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

impl<'a> Deserialize<'a> for Uv {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        let u = de.read_f32()?;
        let v = de.read_f32()?;

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

impl<'a> Deserialize<'a> for NxfArray {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        let min_x = de.read_f32()?;
        let min_y = de.read_f32()?;
        let min_z = de.read_f32()?;

        let num_uvs = de.read_u32()?;

        let max_x = de.read_f32()?;
        let max_y = de.read_f32()?;
        let max_z = de.read_f32()?;

        let num_normals = de.read_u32()?;

        let c_x = de.read_f32()?;
        let c_y = de.read_f32()?;
        let c_z = de.read_f32()?;
        let radius = de.read_f32()?;

        let num_verts = de.read_u32()?;
        let num_cols = de.read_u32()?;
        let max_verts = de.read_u32()?;
        let max_normals = de.read_u32()?;
        let max_cols = de.read_u32()?;
        let max_uvs = de.read_u32()?;

        let verts = de.deserialize::<SinglePointer<SizedList<Vec3>>>((num_verts as usize, ()))?;

        let normals = de.deserialize::<SinglePointer<SizedList<Vec3>>>((num_normals as usize, ()))?;

        let colors = de.deserialize::<SinglePointer<SizedList<Color>>>((num_cols as usize, ()))?;

        let uvs = de.deserialize::<SinglePointer<SizedList<Uv>>>((num_uvs as usize, ()))?;

        let flags = de.read_u32()?;

        let _pad1 = de.read_u32()?;
        let _pad2 = de.read_u32()?;

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

impl<'a> Deserialize<'a> for NxfColLitTri {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        Ok(NxfColLitTri {
            v0: de.read_u16()?,
            n0: de.read_u16()?,
            c0: de.read_u16()?,
            v1: de.read_u16()?,
            n1: de.read_u16()?,
            c1: de.read_u16()?,
            v2: de.read_u16()?,
            n2: de.read_u16()?,
            c2: de.read_u16()?,
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

impl<'a> Deserialize<'a> for NxfTexLitTri {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        Ok(NxfTexLitTri {
            v0: de.read_u16()?,
            n0: de.read_u16()?,
            c0: de.read_u16()?,
            uv0: de.read_u16()?,
            v1: de.read_u16()?,
            n1: de.read_u16()?,
            c1: de.read_u16()?,
            uv1: de.read_u16()?,
            v2: de.read_u16()?,
            n2: de.read_u16()?,
            c2: de.read_u16()?,
            uv2: de.read_u16()?,
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

impl<'a> Deserialize<'a> for NxfTexUnlitTri {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        Ok(NxfTexUnlitTri {
            v0: de.read_u16()?,
            c0: de.read_u16()?,
            uv0: de.read_u16()?,
            v1: de.read_u16()?,
            c1: de.read_u16()?,
            uv1: de.read_u16()?,
            v2: de.read_u16()?,
            c2: de.read_u16()?,
            uv2: de.read_u16()?,
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

impl<'a> Deserialize<'a> for NxfColUnlitTri {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        Ok(NxfColUnlitTri {
            v0: de.read_u16()?,
            c0: de.read_u16()?,
            v1: de.read_u16()?,
            c1: de.read_u16()?,
            v2: de.read_u16()?,
            c2: de.read_u16()?,
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

impl<'a> Deserialize<'a> for NxfTexLitEnvTri {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        Ok(NxfTexLitEnvTri {
            v0: de.read_u16()?,
            n0: de.read_u16()?,
            c0: de.read_u16()?,
            uv0: de.read_u16()?,
            m0: de.read_u16()?,
            v1: de.read_u16()?,
            n1: de.read_u16()?,
            c1: de.read_u16()?,
            uv1: de.read_u16()?,
            m1: de.read_u16()?,
            v2: de.read_u16()?,
            n2: de.read_u16()?,
            c2: de.read_u16()?,
            uv2: de.read_u16()?,
            m2: de.read_u16()?,
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

impl<'a> Deserialize<'a> for NxfColLitEnvTri {
    type Output = Self;
    type State = ();

    fn deserialize<R>(de: &mut Deserializer<R>, _state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        Ok(NxfColLitEnvTri {
            v0: de.read_u16()?,
            n0: de.read_u16()?,
            c0: de.read_u16()?,
            m0: de.read_u16()?,
            v1: de.read_u16()?,
            n1: de.read_u16()?,
            c1: de.read_u16()?,
            m1: de.read_u16()?,
            v2: de.read_u16()?,
            n2: de.read_u16()?,
            c2: de.read_u16()?,
            m2: de.read_u16()?,
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

impl<'a> Deserialize<'a> for NxfFaces {
    type Output = Self;
    type State = (u8, u32);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self>
    where
        R: Read + Seek,
    {
        let facelist_type = state.0;
        let num = state.1;

        match facelist_type {
            6 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(de.deserialize::<NxfColLitTri>(())?);
                }
                Ok(NxfFaces::ColLitTri(faces))
            }
            8 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(de.deserialize::<NxfTexLitTri>(())?);
                }
                Ok(NxfFaces::TexLitTri(faces))
            }
            10 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(de.deserialize::<NxfTexUnlitTri>(())?);
                }
                Ok(NxfFaces::TexUnlitTri(faces))
            }
            11 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(de.deserialize::<NxfColUnlitTri>(())?);
                }
                Ok(NxfFaces::ColUnlitTri(faces))
            }
            20 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(de.deserialize::<NxfTexLitEnvTri>(())?);
                }
                Ok(NxfFaces::TexLitEnvTri(faces))
            }
            21 => {
                let mut faces = Vec::new();
                for _ in 0..num {
                    faces.push(de.deserialize::<NxfColLitEnvTri>(())?);
                }
                Ok(NxfFaces::ColLitEnvTri(faces))
            }
            _ => panic!("Bad face type"),
        }
    }
}

impl NxfFaces {
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
    pub material: Rc<NxfMaterial>,
    pub faces: NxfFaces,
    next_facelist: u64, // XXX: needed (for now) so I can read a list of these
    pub display_list: u32,
    pub display_list_size: u32,
}

impl<'a> Deserialize<'a> for NxfFacelist {
    type Output = Self;
    type State = (&'a mut SharedPointerDeserializeState<NxfMaterial>, &'a mut SharedPointerDeserializeState<String>);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self::Output>
    where
        R: Read + Seek,
    {
        let flags = de.read_u16()?;
        let facelist_type = de.read_u8()?;
        let attribs = de.read_u8()?;
        let _pad = de.read_u32()?;

        let material = de.deserialize::<SharedPointer<NxfMaterial>>(state)?;

        let num_faces = de.read_u32()?;
        let faces = de.deserialize::<SinglePointer<NxfFaces>>((facelist_type, num_faces))?;

        let next_facelist = de.read_u32()? as u64;

        let display_list = de.read_u32()?;
        let display_list_size = de.read_u32()?;

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
}

struct NxfFacelistListDeserializer;

impl<'a> Deserialize<'a> for NxfFacelistListDeserializer {
    type Output = Vec<NxfFacelist>;
    type State = (&'a mut SharedPointerDeserializeState<NxfMaterial>, &'a mut SharedPointerDeserializeState<String>);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self::Output>
    where
        R: Read + Seek,
    {
        let mut facelists = Vec::new();
        let mut data_addr = de.read_u32()? as u64;
        let save_addr = de.inner().seek(SeekFrom::Current(0))?;

        while data_addr != 0 {
            de.inner().seek(SeekFrom::Start(data_addr))?;
            let facelist = de.deserialize::<NxfFacelist>((state.0, state.1))?;
            data_addr = facelist.next_facelist;
            facelists.push(facelist);
        }

        de.inner().seek(SeekFrom::Start(save_addr))?;

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

impl<'a> Deserialize<'a> for NxfFacelistSet {
    type Output = Self;
    type State = (&'a mut SharedPointerDeserializeState<NxfMaterial>, &'a mut SharedPointerDeserializeState<String>);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self::Output>
    where
        R: Read + Seek,
    {
        let flags = de.read_u32()?;
        let _pad = de.read_u32()?;

        let _num_lists = de.read_u32()?;
        let facelists = de.deserialize::<NxfFacelistListDeserializer>(state)?;

        // TODO: de.mat palettes
        let _mat_palette_offset = de.read_u32()?;

        Ok(NxfFacelistSet {
            flags: flags,
            facelists: facelists,
            mat_palette: None,
        })
    }
}

struct NxfFacelistSetListDeserializer;

impl<'a> Deserialize<'a> for NxfFacelistSetListDeserializer {
    type Output = Vec<NxfFacelistSet>;
    type State = (&'a mut SharedPointerDeserializeState<NxfMaterial>, &'a mut SharedPointerDeserializeState<String>);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self::Output>
    where
        R: Read + Seek,
    {
        let mut facelistsets = Vec::new();
        let mut data_addr = de.read_u32()? as u64;
        let save_addr = de.inner().seek(SeekFrom::Current(0))?;

        while data_addr != 0 {
            de.inner().seek(SeekFrom::Start(data_addr))?;
            let facelistset = de.deserialize::<NxfFacelistSet>((state.0, state.1))?;
            data_addr = de.read_u32()? as u64;
            facelistsets.push(facelistset);
        }

        de.inner().seek(SeekFrom::Start(save_addr))?;

        Ok(facelistsets)
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
    pub strings: Vec<Rc<String>>,
    pub materials: Vec<Rc<NxfMaterial>>,
    pub arrays: NxfArray,
    pub facelist_sets: Vec<NxfFacelistSet>,
    pub display_list: u32,
    pub display_list_size: u32,
}

impl<'a> Deserialize<'a> for NxfObjGeom {
    type Output = Self;
    type State = (&'a mut SharedPointerDeserializeState<NxfMaterial>, &'a mut SharedPointerDeserializeState<String>);

    fn deserialize<R>(de: &mut Deserializer<R>, state: Self::State) -> IoResult<Self::Output>
    where
        R: Read + Seek,
    {
        let mut id = [0; 4];
        de.inner().read_exact(&mut id)?;
        let endian = de.read_u32()?;
        let version = de.read_f32()?;
        let flags = de.read_u32()?;
        let alpha_mode = de.read_u32()?;
        let env_map_alpha_mode = de.read_u32()?;

        let num_strings = de.read_u16()? as usize;
        let _pad = de.read_u16()?;
        let strings_offset = de.read_u32()?;
        let strings = de.deserialize::<SizedList<SharedPointer<TerminatedString>>>((num_strings, (state.1, ())))?;

        let material_offset = de.read_u32()?;
        let materials = de.deserialize::<NxfMaterialListDeserializer>(state)?;

        let arrays = de.deserialize::<SinglePointer<NxfArray>>(())?;

        let facelist_sets = de.deserialize::<NxfFacelistSetListDeserializer>(state)?;

        let display_list = de.read_u32()?;
        let display_list_size = de.read_u32()?;
        let _expanded = de.read_u32()?; // TODO: read more geoms
        let _pad1 = de.read_u32()?;
        let _pad2 = de.read_u32()?;
        let _pad3 = de.read_u32()?;

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