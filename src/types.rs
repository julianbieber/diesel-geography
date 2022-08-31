//! Rust Types.

use crate::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use postgis::ewkb::LineString;
use postgis::ewkb::Point;
use std::convert::From;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(sql_type = Geography)]
pub struct GeogPoint {
    pub x: f64, // lon
    pub y: f64, // lat
    pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
    fn from(p: Point) -> Self {
        let Point { x, y, srid } = p;
        Self { x, y, srid }
    }
}
impl From<GeogPoint> for Point {
    fn from(p: GeogPoint) -> Self {
        let GeogPoint { x, y, srid } = p;
        Self { x, y, srid }
    }
}

impl FromSql<Geography, Pg> for GeogPoint {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = value.as_bytes();
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for GeogPoint {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(sql_type = Geography)]
pub struct GeogLineString {
    pub points: Vec<GeogPoint>,
    pub srid: Option<i32>,
}

impl From<LineString> for GeogLineString {
    fn from(p: LineString) -> Self {
        let LineString { points, srid } = p;
        let geog_points = points.into_iter().map(GeogPoint::from).collect();
        Self {
            points: geog_points,
            srid,
        }
    }
}
impl From<GeogLineString> for LineString {
    fn from(p: GeogLineString) -> Self {
        let GeogLineString { points, srid } = p;
        let gis_points = points.into_iter().map(Point::from).collect();
        Self {
            points: gis_points,
            srid,
        }
    }
}

impl FromSql<Geography, Pg> for GeogLineString {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;
        let bytes = value.as_bytes();
        let mut rdr = Cursor::new(bytes);
        Ok(LineString::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for GeogLineString {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbLineString, EwkbWrite};
        LineString::from(self.clone()).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
