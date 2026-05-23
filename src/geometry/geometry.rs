use super::point::Point;

pub trait Entity {
    fn start(&self) -> Point;
    fn end(&self) -> Point;
    fn revert(&self) -> Box<dyn Entity>;
    fn to_gcode(&self, speed: f64, goto_start: bool) -> String;
}

// impl From<DxfEntity> for Box<dyn Entity> {
//     fn from(dxf_entity: DxfEntity) -> Self {
//         match &dxf_entity.specific{
//             dxf::entities::EntityType::Line(line) => Box::new(super::line::Line::from(line)),
//             dxf::entities::EntityType::Arc(arc) => Box::new(super::arc::Arc::from(arc)),
//             _ => panic!("Unsupported entity type: {:?}", dxf_entity.specific),
//         }
//     }
// }