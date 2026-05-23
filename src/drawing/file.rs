use dxf::Drawing;

pub struct DxfFile{
    drawing: Drawing,
}


impl DxfFile {
    pub fn new(filename: String) -> Result<Self, dxf::DxfError> {
        let drawing = Drawing::load_file(filename)?;
        Ok(Self{drawing})
    }

    pub fn display(&self) {
        for entity in self.drawing.entities() {
            match &entity.specific {
                dxf::entities::EntityType::Line(line) => {
                    // let line_entity = super::line::Line::from(line);
                    println!("{:?}", line);
                },
                dxf::entities::EntityType::Circle(circle) => {
                    // let circle_entity = super::circle::Circle::from(circle);
                    println!("{:?}", circle);
                },
                dxf::entities::EntityType::Arc(arc) => {
                    // let arc_entity = super::arc::Arc::from(arc);
                    println!("{:?}", arc);
                },
                _ => panic!("Unsupported entity type: {:?}", entity.specific),
            }
        }
    }
}