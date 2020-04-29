use std::io::Write;

use nxf::{NxfObjGeom, NxfFaces};
use xml::EmitterConfig;
use xml::writer::{EventWriter, Error as EmitterError};
use xml::writer::events::XmlEvent;

pub struct Nxf2Collada<W> {
    name: String,
    writer: EventWriter<W>,
    nxf: NxfObjGeom,
}

impl<W> Nxf2Collada<W>
    where W: Write,
{
    pub fn new(name: String, nxf: NxfObjGeom, write: W) -> Nxf2Collada<W> {
        Nxf2Collada {
            name: name,
            writer: EventWriter::new_with_config(write, EmitterConfig::new().perform_indent(true)),
            nxf: nxf,
        }
    }

    pub fn write_collada(&mut self) -> Result<(), EmitterError> {
        self.write_start()?;
        self.write_library_effects()?;
        self.write_library_images()?;
        self.write_library_materials()?;
        self.write_library_geometries()?;
        self.write_library_nodes()?;
        self.write_library_visual_scenes()?;
        self.write_scene()?;
        self.write_end()
    }

    fn write_start(&mut self) -> Result<(), EmitterError> {
        self.writer.write(
            XmlEvent::start_element("COLLADA")
                .attr("xmlns", "http://www.collada.org/2005/11/COLLADASchema")
                .attr("version", "1.4.1")
        )?;
        self.writer.write(XmlEvent::start_element("asset"))?;
        self.writer.write(XmlEvent::start_element("created"))?;
        self.writer.write("2020-04-18T17:41:28")?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::start_element("modified"))?;
        self.writer.write("2020-04-18T17:41:28")?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())
    }

    fn write_library_effects(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_effects"))?;

        for material in self.nxf.materials.iter() {
            self.writer.write(
                XmlEvent::start_element("effect")
                    .attr("id", &(material.tex_name.clone() + "_effect"))
            )?;
            self.writer.write(XmlEvent::start_element("profile_COMMON"))?;
            self.writer.write(
                XmlEvent::start_element("technique")
                    .attr("sid", &(material.tex_name.clone() + "_technique"))
            )?;

            self.writer.write(
                XmlEvent::start_element("newparam")
                    .attr("sid", &(material.tex_name.clone() + "_surface"))
            )?;
            self.writer.write(
                XmlEvent::start_element("surface")
                    .attr("type", "2D")
            )?;
            self.writer.write(XmlEvent::start_element("init_from"))?;
            self.writer.write((material.tex_name.clone() + "_image").as_str())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;

            self.writer.write(
                XmlEvent::start_element("newparam")
                    .attr("sid", &(material.tex_name.clone() + "_sampler"))
            )?;
            self.writer.write(XmlEvent::start_element("sampler2D"))?;
            self.writer.write(XmlEvent::start_element("source"))?;
            self.writer.write((material.tex_name.clone() + "_surface").as_str())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;

            self.writer.write(XmlEvent::start_element("lambert"))?;
            self.writer.write(XmlEvent::start_element("diffuse"))?;
            self.writer.write(
                XmlEvent::start_element("texture")
                    .attr("texture", &(material.tex_name.clone() + "_sampler"))
                    .attr("texcoord", "nxf_uvs")
            )?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;

            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
        }

        self.writer.write(XmlEvent::end_element())
    }

    fn write_library_images(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_images"))?;

        for material in self.nxf.materials.iter() {
            self.writer.write(
                XmlEvent::start_element("image")
                    .attr("id", &(material.tex_name.clone() + "_image"))
            )?;
            self.writer.write(XmlEvent::start_element("init_from"))?;
            self.writer.write((material.tex_name.clone() + ".png").as_str())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
        }

        self.writer.write(XmlEvent::end_element())
    }

    fn write_library_materials(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_materials"))?;

        for material in self.nxf.materials.iter() {
            self.writer.write(
                XmlEvent::start_element("material")
                    .attr("id", &(material.tex_name.clone() + "_material"))
            )?;
            self.writer.write(
                XmlEvent::start_element("instance_effect")
                    .attr("url", (String::from("#") + &material.tex_name + "_effect").as_str())
            )?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
        }

        self.writer.write(XmlEvent::end_element())
    }

    fn write_library_geometries(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_geometries"))?;
        self.writer.write(
            XmlEvent::start_element("geometry")
                .attr("id", (self.name.clone() + "_geometry").as_str())
                .attr("name", (self.name.clone() + "_geometry").as_str())
        )?;
        self.writer.write(XmlEvent::start_element("mesh"))?;

        // vertex source
        self.writer.write(
            XmlEvent::start_element("source")
                .attr("id", "vertex_source")
        )?;

        self.writer.write(
            XmlEvent::start_element("float_array")
                .attr("id", "vertex_array")
                .attr("count", (self.nxf.arrays.verts.len() * 3).to_string().as_str())
        )?;
        let mut vertex_data = String::new();
        for vertex in self.nxf.arrays.verts.iter() {
            vertex_data += &format!("{} {} {} ", vertex.x, -vertex.y, -vertex.z);
        }
        self.writer.write(vertex_data.as_str())?;
        self.writer.write(XmlEvent::end_element())?;

        self.writer.write(XmlEvent::start_element("technique_common"))?;
        self.writer.write(
            XmlEvent::start_element("accessor")
                .attr("source", "#vertex_array")
                .attr("count", (self.nxf.arrays.verts.len()).to_string().as_str())
                .attr("stride", "3")
        )?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "X")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "Y")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "Z")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;

        self.writer.write(XmlEvent::end_element())?;

        // color source
        self.writer.write(
            XmlEvent::start_element("source")
                .attr("id", "color_source")
        )?;

        self.writer.write(
            XmlEvent::start_element("float_array")
                .attr("id", "color_array")
                .attr("count", (self.nxf.arrays.colors.len() * 4).to_string().as_str())
        )?;
        let mut color_data = String::new();
        for color in self.nxf.arrays.colors.iter() {
            color_data += &format!("{} {} {} {} ",
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0
            );
        }
        self.writer.write(color_data.as_str())?;
        self.writer.write(XmlEvent::end_element())?;

        self.writer.write(XmlEvent::start_element("technique_common"))?;
        self.writer.write(
            XmlEvent::start_element("accessor")
                .attr("source", "#color_array")
                .attr("count", (self.nxf.arrays.colors.len()).to_string().as_str())
                .attr("stride", "4")
        )?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "R")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "G")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "B")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(
            XmlEvent::start_element("param")
                .attr("name", "A")
                .attr("type", "float")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;

        self.writer.write(XmlEvent::end_element())?;

        // uv source
        if self.nxf.arrays.uvs.len() != 0 {
            self.writer.write(
                XmlEvent::start_element("source")
                    .attr("id", "uv_source")
            )?;

            self.writer.write(
                XmlEvent::start_element("float_array")
                    .attr("id", "uv_array")
                    .attr("count", (self.nxf.arrays.uvs.len() * 2).to_string().as_str())
            )?;
            let mut uv_data = String::new();
            for uv in self.nxf.arrays.uvs.iter() {
                uv_data += &format!("{} {} ", uv.u, 1.0 - uv.v);
            }
            self.writer.write(uv_data.as_str())?;
            self.writer.write(XmlEvent::end_element())?;

            self.writer.write(XmlEvent::start_element("technique_common"))?;
            self.writer.write(
                XmlEvent::start_element("accessor")
                    .attr("source", "#uv_array")
                    .attr("count", (self.nxf.arrays.uvs.len()).to_string().as_str())
                    .attr("stride", "2")
            )?;
            self.writer.write(
                XmlEvent::start_element("param")
                    .attr("name", "S")
                    .attr("type", "float")
            )?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(
                XmlEvent::start_element("param")
                    .attr("name", "T")
                    .attr("type", "float")
            )?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;

            self.writer.write(XmlEvent::end_element())?;
        }

        // TODO: Normals

        self.writer.write(
            XmlEvent::start_element("vertices")
                .attr("id", "vertices")
        )?;
        self.writer.write(
            XmlEvent::start_element("input")
                .attr("semantic", "POSITION")
                .attr("source", "#vertex_source")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;

        for facelist_set in self.nxf.facelist_sets.iter() {
            for facelist in facelist_set.facelists.iter() {
                self.writer.write(
                    XmlEvent::start_element("triangles")
                        .attr("count", facelist.faces.len().to_string().as_str())
                        .attr("material", (facelist.material.tex_name.clone() + "_symbol").as_str())
                )?;

                self.writer.write(
                    XmlEvent::start_element("input")
                        .attr("offset", "0")
                        .attr("semantic", "VERTEX")
                        .attr("source", "#vertices")
                )?;
                self.writer.write(XmlEvent::end_element())?;
                self.writer.write(
                    XmlEvent::start_element("input")
                        .attr("offset", "1")
                        .attr("semantic", "COLOR")
                        .attr("source", "#color_source")
                )?;
                self.writer.write(XmlEvent::end_element())?;

                match &facelist.faces {
                    NxfFaces::ColLitTri(_faces) => {
                        unimplemented!()
                    },
                    NxfFaces::TexLitTri(_faces) => {
                        unimplemented!()
                    },
                    NxfFaces::TexUnlitTri(faces) => {
                        self.writer.write(
                            XmlEvent::start_element("input")
                                .attr("offset", "2")
                                .attr("semantic", "TEXCOORD")
                                .attr("source", "#uv_source")
                        )?;
                        self.writer.write(XmlEvent::end_element())?;

                        self.writer.write(XmlEvent::start_element("p"))?;
                        let mut face_data = String::new();
                        for face in faces {
                            face_data += &format!("{} {} {} {} {} {} {} {} {} ",
                                face.v0,
                                face.c0,
                                face.uv0,
                                face.v1,
                                face.c1,
                                face.uv1,
                                face.v2,
                                face.c2,
                                face.uv2,
                            );
                        }
                        self.writer.write(face_data.as_str())?;
                        self.writer.write(XmlEvent::end_element())?;
                    },
                    NxfFaces::ColUnlitTri(faces) => {
                        self.writer.write(XmlEvent::start_element("p"))?;
                        let mut face_data = String::new();
                        for face in faces {
                            face_data += &format!("{} {} {} {} {} {} ",
                                face.v0,
                                face.c0,
                                face.v1,
                                face.c1,
                                face.v2,
                                face.c2,
                            );
                        }
                        self.writer.write(face_data.as_str())?;
                        self.writer.write(XmlEvent::end_element())?;
                    },
                    NxfFaces::TexLitEnvTri(_faces) => {
                        unimplemented!()
                    },
                    NxfFaces::ColLitEnvTri(_faces) => {
                        unimplemented!()
                    },
                }

                self.writer.write(XmlEvent::end_element())?;
            }
        }

        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())
    }

    fn write_library_nodes(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_nodes"))?;
        self.writer.write(
            XmlEvent::start_element("node")
                .attr("id", "main_node")
        )?;
        self.writer.write(
            XmlEvent::start_element("instance_geometry")
                .attr("url", (String::from("#") + &self.name + "_geometry").as_str())
        )?;

        for material in self.nxf.materials.iter() {
            self.writer.write(XmlEvent::start_element("bind_material"))?;
            self.writer.write(XmlEvent::start_element("technique_common"))?;
            self.writer.write(
                XmlEvent::start_element("instance_material")
                    .attr("symbol", (material.tex_name.clone() + "_symbol").as_str())
                    .attr("target", (String::from("#") + &material.tex_name + "_material").as_str())
            )?;
            self.writer.write(
                XmlEvent::start_element("bind_vertex_input")
                    .attr("semantic", "nxf_uvs")
                    .attr("input_semantic", "TEXCOORD")
            )?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
            self.writer.write(XmlEvent::end_element())?;
        }

        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())
    }

    fn write_library_visual_scenes(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_visual_scenes"))?;
        self.writer.write(
            XmlEvent::start_element("visual_scene")
                .attr("id", "visual_scene")
        )?;
        self.writer.write(
            XmlEvent::start_element("node")
                .attr("name", &self.name)
        )?;
        self.writer.write(
            XmlEvent::start_element("instance_node")
                .attr("url", "#main_node")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())
    }

    fn write_scene(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("scene"))?;
        self.writer.write(
            XmlEvent::start_element("instance_visual_scene")
                .attr("url", "#visual_scene")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())
    }

    fn write_end(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::end_element())
    }
}