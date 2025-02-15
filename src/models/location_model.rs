use serde::{Deserialize, Serialize};
use surrealdb::sql::{Geometry, Value};

#[derive(Deserialize, Serialize)]
pub struct Location {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinates: Option<Geometry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FeatureGeometry {
    Point(Point),
    LineString(LineString),
    Polygon(Polygon),
    MultiPoint(MultiPoint),
    MultiLineString(MultiLineString),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub struct Feature {
    pub geometry: FeatureGeometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Value>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineString {
    pub coordinates: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPoint {
    pub coordinates: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiLineString {
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygon {
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryCollection {
    pub geometries: Vec<FeatureGeometry>,
}
