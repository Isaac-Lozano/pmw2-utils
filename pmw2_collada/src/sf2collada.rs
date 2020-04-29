use std::io::Write;

use sf::{SceneTemplate, ScenePlacementData, SceneGeomFormat};
use xml::EmitterConfig;
use xml::writer::{EventWriter, Error as EmitterError};
use xml::writer::events::XmlEvent;

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

    fn write_library_visual_scenes(&mut self) -> Result<(), EmitterError> {
        self.writer.write(XmlEvent::start_element("library_visual_scenes"))?;
        self.writer.write(
            XmlEvent::start_element("visual_scene")
                .attr("id", "visual_scene")
        )?;

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