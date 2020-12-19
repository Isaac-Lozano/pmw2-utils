use std::io::Write;

use sf::{SceneTemplate, ScenePlacementData, SceneGeomFormat};
use xml::EmitterConfig;
use xml::writer::{EventWriter, Error as EmitterError};
use xml::writer::events::XmlEvent;

use crate::matrix::Matrix;

pub struct Sf2Collada<W> {
    writer: EventWriter<W>,
    sf: SceneTemplate,
}

impl<W> Sf2Collada<W>
    where W: Write,
{
    pub fn new(sf: SceneTemplate, write: W) -> Sf2Collada<W> {
        Sf2Collada {
            writer: EventWriter::new_with_config(write, EmitterConfig::new().perform_indent(true)),
            sf: sf,
        }
    }

    pub fn write_collada(&mut self) -> Result<(), EmitterError> {
        self.write_start()?;
        self.write_library_nodes()?;
        self.write_library_visual_scenes()?;
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

    fn write_library_nodes(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_nodes"))?;

        self.writer.write(
            XmlEvent::start_element("node")
                .attr("id", "points")
        )?;
        for clump in self.sf.clumps.iter() {
            for placement in clump.placements.iter() {
                match placement.data {
                    ScenePlacementData::Point(_) => {
                        self.writer.write(
                            XmlEvent::start_element("node")
                                .attr("name", &placement.geom_name)
                        )?;
                        self.writer.write(XmlEvent::start_element("translate"))?;
                        self.writer.write(format!("{} {} {}",
                            placement.x_pos,
                            -placement.y_pos,
                            -placement.z_pos,
                        ).as_str())?;
                        self.writer.write(XmlEvent::end_element())?;
                        self.writer.write(
                            XmlEvent::start_element("instance_geometry")
                                .attr("url", "sphere.dae#Sphere-mesh")
                        )?;
                        self.writer.write(XmlEvent::end_element())?;
                        self.writer.write(XmlEvent::end_element())?;
                    }
                    ScenePlacementData::BoundingBox{ min: (minx, miny, minz, _minw), max: (maxx, maxy, maxz, _maxw), .. } => {
                        self.writer.write(
                            XmlEvent::start_element("node")
                                .attr("name", &placement.geom_name)
                        )?;

                        self.writer.write(XmlEvent::start_element("matrix"))?;
                        let mut mat = Matrix::new();
                        let c_x = ((minx + maxx) / 2.0) + placement.x_pos;
                        let c_y = ((miny + maxy) / 2.0) + placement.y_pos;
                        let c_z = ((minz + maxz) / 2.0) + placement.z_pos;
                        mat = mat.translate((c_x, -c_y, -c_z, placement.w_pos));
                        mat = mat.scale(((maxx - minx) / 2.0, (maxy - miny) / 2.0, (maxz - minz) / 2.0));
                        mat = mat.rot_yxz((placement.x_rot, -placement.y_rot, -placement.z_rot));
                        self.writer.write(format!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                            mat.0[0x0], mat.0[0x1], mat.0[0x2], mat.0[0x3],
                            mat.0[0x4], mat.0[0x5], mat.0[0x6], mat.0[0x7],
                            mat.0[0x8], mat.0[0x9], mat.0[0xa], mat.0[0xb],
                            mat.0[0xc], mat.0[0xd], mat.0[0xe], mat.0[0xf],
                        ).as_str())?;
                        self.writer.write(XmlEvent::end_element())?;

                        self.writer.write(
                            XmlEvent::start_element("instance_geometry")
                                .attr("url", "cube.dae#Cube-mesh")
                        )?;
                        self.writer.write(XmlEvent::end_element())?;

                        self.writer.write(XmlEvent::end_element())?;
                    }
                    _ => {}
                }
            }
        }
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
                .attr("name", "__points")
        )?;
        self.writer.write(
            XmlEvent::start_element("instance_node")
                .attr("url", "#points")
        )?;
        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())?;

        for clump in self.sf.clumps.iter() {
            for placement in clump.placements.iter() {
                match placement.data {
                    ScenePlacementData::Static(SceneGeomFormat::Nxf) |
                    ScenePlacementData::StaticInst(SceneGeomFormat::Nxf) |
                    ScenePlacementData::Ground(SceneGeomFormat::Nxf) |
                    ScenePlacementData::GroundVU1(SceneGeomFormat::Nxf) |
                    ScenePlacementData::Sky(SceneGeomFormat::Nxf) => {
                        self.writer.write(
                            XmlEvent::start_element("node")
                                .attr("name", &placement.geom_name)
                        )?;
                        self.writer.write(XmlEvent::start_element("translate"))?;
                        self.writer.write(format!("{} {} {}",
                            placement.x_pos,
                            -placement.y_pos,
                            -placement.z_pos,
                        ).as_str())?;
                        self.writer.write(XmlEvent::end_element())?;
                        self.writer.write(
                            XmlEvent::start_element("instance_node")
                                .attr("url", (placement.geom_name.clone() + ".dae#main_node").as_str())
                        )?;
                        self.writer.write(XmlEvent::end_element())?;
                        self.writer.write(XmlEvent::end_element())?;
                    }
                    _ => {}
                }
            }
        }

        self.writer.write(XmlEvent::end_element())?;
        self.writer.write(XmlEvent::end_element())
    }

    fn write_end(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::end_element())
    }
}