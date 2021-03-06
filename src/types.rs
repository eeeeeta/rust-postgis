//
// Copyright (c) Pirmin Kalberer. All rights reserved.
//

pub trait Point {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn opt_z(&self) -> Option<f64> {
        None
    }
    fn opt_m(&self) -> Option<f64> {
        None
    }
}

pub trait LineString<'a> {
    type ItemType: 'a + Point;
    type Iter: Iterator<Item=&'a Self::ItemType>;
    fn points(&'a self) -> Self::Iter;
}

pub trait Polygon<'a> {
    type ItemType: 'a + LineString<'a>;
    type Iter: Iterator<Item=&'a Self::ItemType>;
    fn rings(&'a self) -> Self::Iter;
}

pub trait MultiPoint<'a> {
    type ItemType: 'a + Point;
    type Iter: Iterator<Item=&'a Self::ItemType>;
    fn points(&'a self) -> Self::Iter;
}

pub trait MultiLineString<'a> {
    type ItemType: 'a + LineString<'a>;
    type Iter: Iterator<Item=&'a Self::ItemType>;
    fn lines(&'a self) -> Self::Iter;
}

pub trait MultiPolygon<'a> {
    type ItemType: 'a + Polygon<'a>;
    type Iter: Iterator<Item=&'a Self::ItemType>;
    fn polygons(&'a self) -> Self::Iter;
}
