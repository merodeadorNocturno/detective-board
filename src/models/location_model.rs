// detective-board/src/models/location_model.rs
use geo_types::Point;
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Geometry, RecordId}; // Import geo-types

#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    pub id: RecordId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_geometry")] // Use custom deserializer
    pub coordinates: Option<Geometry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationJson {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_geometry")] //Use custom deserializer
    pub coordinates: Option<Geometry>,
}

fn deserialize_geometry<'de, D>(deserializer: D) -> Result<Option<Geometry>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Attempt to deserialize as a Point first
    if let Ok(point) = Point::deserialize(deserializer) {
        return Ok(Some(Geometry::Point(point)));
    }

    // Handle other geometry types if needed
    Ok(None)
}
