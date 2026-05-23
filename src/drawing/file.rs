use dxf::Drawing;
use dxf::entities::*;
use glob::Pattern;

use crate::models::entity::Entity;
use crate::models::entity_set::EntitySet;
use crate::models::{
    arc::Arc,
    dxf::Layered,
    line::Line,
    point::Point,
    filter::Filtered,
};

use crate::errors::load::LoaderError;


pub struct DxfFile{
    drawing: Option<Drawing>,

    lines: Vec<Line>,
    arcs: Vec<Arc>,
    points: Vec<Point>,
}

impl Clone for DxfFile {
    fn clone(&self) -> Self {
        DxfFile {
            drawing: None,
            lines: self.lines.clone(),
            arcs: self.arcs.clone(),
            points: self.points.clone(),
        }
    }
}

impl DxfFile {
    pub fn new(filename: String) -> Result<Self, dxf::DxfError> {
        let drawing = Drawing::load_file(filename)?;
        Ok(Self{
            drawing: Some(drawing),
            lines: Vec::new(),
            arcs: Vec::new(),
            points: Vec::new(),
        })
    }

    pub fn load(&mut self) -> Result<(), LoaderError> {
        match &self.drawing {
            None => Err(LoaderError::DrawingUnavailable()),
            Some(drawing) => {
                for entity in drawing.entities() {
                    match &entity.specific {
                        EntityType::Line(line) => {
                            let layered_line = Layered { layer: entity.common.layer.clone(), entity: line };
                            self.lines.push(Line::from(layered_line));
                        },
                        EntityType::Polyline(polyline) => {
                            let points = polyline.vertices()
                                .map(|v| Point::new(v.location.x, v.location.y, v.location.z, entity.common.layer.clone()))
                                .collect::<Vec<_>>();
                            for i in 0..points.len() - 1 {
                                self.lines.push(Line::new(
                                    points[i].clone(), 
                                    points[i + 1].clone(), 
                                    entity.common.layer.clone(),
                                ));
                            }
                        },
                        EntityType::LwPolyline(lw_polyline) => {
                            let points = lw_polyline.vertices
                                .iter()
                                .map(|v| Point::new(v.x, v.y, 0.0, entity.common.layer.clone()))
                                .collect::<Vec<_>>();
                            for i in 0..points.len() - 1 {
                                self.lines.push(Line::new(
                                    points[i].clone(), 
                                    points[i + 1].clone(), 
                                    entity.common.layer.clone(),
                                ));
                            }
                        },
                        EntityType::MLine(m_line) => {
                            let points = m_line.vertices.iter().map(|v| Point::new(v.x, v.y, v.z, entity.common.layer.clone())).collect::<Vec<_>>();
                            for i in 0..points.len() - 1 {
                                self.lines.push(Line::new(
                                    points[i].clone(), 
                                    points[i + 1].clone(), 
                                    entity.common.layer.clone(),
                                ));
                            }
                        },
                        EntityType::Arc(arc) => {
                            self.arcs.push(Arc::from(Layered { layer: entity.common.layer.clone(), entity: arc }));
                        },
                        EntityType::Vertex(vertex) => {
                            self.points.push(Point::new(
                                vertex.location.x, 
                                vertex.location.y, 
                                vertex.location.z, 
                                entity.common.layer.clone(),
                            ));
                        },
                        EntityType::ModelPoint(model_point) => {
                            self.points.push(Point::new(
                                model_point.location.x, 
                                model_point.location.y, 
                                model_point.location.z, 
                                entity.common.layer.clone(),
                            ));
                        },
                        _ => (),
                    };
                }

                Ok(())
            },
        }
        
    }

    pub fn filter_layer(&mut self, layer_criteria: String, entity_filter: Vec<String>) -> Result<Self, LoaderError> {
        let mut filtered = self.clone();

        match Pattern::new(layer_criteria.as_str()) {
            Ok(pattern) => {
                filtered.lines = filter_by_criteria(filtered.lines, &pattern, &entity_filter);
                filtered.arcs = filter_by_criteria(filtered.arcs, &pattern, &entity_filter);
                filtered.points = filter_by_criteria(filtered.points, &pattern, &entity_filter);

                Ok(filtered)
            },
            Err(_) => return Err(LoaderError::WrongLayerPattern(layer_criteria)),
        }
    }


    pub fn display(&self) {
        for line in &self.lines {
            println!("#{} => Line: start=({:.2}, {:.2}, {:.2}), end=({:.2}, {:.2}, {:.2})",
                line.layer,
                line.start.x, line.start.y, line.start.z,
                line.end.x, line.end.y, line.end.z);
        }
        for arc in &self.arcs {
            println!("#{} => Arc: center=({:.2}, {:.2}, {:.2}), radius={:.2}, start_angle={:.2}, end_angle={:.2}, clockwise={}",
                arc.layer,
                arc.center.x, arc.center.y, arc.center.z,
                arc.radius, arc.start_angle, arc.end_angle, arc.clockwise);
        }
        for point in &self.points {
            println!("#{} => Point: ({:.2}, {:.2}, {:.2})", point.layer, point.x, point.y, point.z);
        }
    }

    pub fn entities(&self) -> EntitySet {
        let mut output = EntitySet::default();
        for arc in &self.arcs {
            output.push(Entity::Arc(arc.clone()));
        }

        for line in &self.lines {
            output.push(Entity::Line(line.clone()));
        }

        for point in &self.points {
            output.push(Entity::Point(point.clone()));
        }
        
        output
    }
}


fn match_criteria(name: &str, pattern: &Pattern) -> bool {
    pattern.matches(name)
}

fn has_name(name: &String, list: &Vec<String>) -> bool {
    if list.is_empty() {
        true
    } else {
        list.contains(name)
    }
}

fn filter_by_criteria<T: Filtered>(data: Vec<T>, pattern: &Pattern, entity_filter: &Vec<String>) -> Vec<T> {
    data.into_iter()
        .filter(|item| {
            match_criteria(&item.layer(), &pattern) && has_name(&item.entity_type(), entity_filter)
        })
        .collect()
}