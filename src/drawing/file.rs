use dxf::Drawing;
use crate::models::{
    arc, line::Line, point::Point
};
use dxf::entities::*;

pub struct DxfFile{
    drawing: Drawing,

    lines: Vec<Line>,
    arcs: Vec<arc::Arc>,
    points: Vec<Point>,
}


impl DxfFile {
    pub fn new(filename: String) -> Result<Self, dxf::DxfError> {
        let drawing = Drawing::load_file(filename)?;
        Ok(Self{
            drawing,
            lines: Vec::new(),
            arcs: Vec::new(),
            points: Vec::new(),
        })
    }

    pub fn load(&mut self) {
        for entity in self.drawing.entities() {
            match &entity.specific {
                EntityType::Line(line) => {
                    self.lines.push(Line::from(line));
                },
                EntityType::Polyline(polyline) => {
                    let points = polyline.vertices().map(|v| Point::new(v.location.x, v.location.y, v.location.z)).collect::<Vec<_>>();
                    for i in 0..points.len() - 1 {
                        self.lines.push(Line::new(points[i].clone(), points[i + 1].clone()));
                    }
                },
                EntityType::LwPolyline(lw_polyline) => {
                    let points = lw_polyline.vertices.iter().map(|v| Point::new(v.x, v.y, 0.0)).collect::<Vec<_>>();
                    for i in 0..points.len() - 1 {
                        self.lines.push(Line::new(points[i].clone(), points[i + 1].clone()));
                    }
                },
                EntityType::MLine(m_line) => {
                    let points = m_line.vertices.iter().map(|v| Point::new(v.x, v.y, v.z)).collect::<Vec<_>>();
                    for i in 0..points.len() - 1 {
                        self.lines.push(Line::new(points[i].clone(), points[i + 1].clone()));
                    }
                },
                EntityType::Arc(arc) => {
                    self.arcs.push(arc::Arc::from(arc));
                },
                EntityType::Vertex(vertex) => {
                    self.points.push(Point::new(vertex.location.x, vertex.location.y, vertex.location.z));
                },
                _ => (),
            };
        }
    }


    pub fn display(&self) {
        for line in &self.lines {
            println!("Line: start=({:.2}, {:.2}, {:.2}), end=({:.2}, {:.2}, {:.2})",
                line.start.x, line.start.y, line.start.z,
                line.end.x, line.end.y, line.end.z);
        }
        for arc in &self.arcs {
            println!("Arc: center=({:.2}, {:.2}, {:.2}), radius={:.2}, start_angle={:.2}, end_angle={:.2}, clockwise={}",
                arc.center.x, arc.center.y, arc.center.z,
                arc.radius, arc.start_angle, arc.end_angle, arc.clockwise);
        }
        for point in &self.points {
            println!("Point: ({:.2}, {:.2}, {:.2})", point.x, point.y, point.z);
        }
    }
}