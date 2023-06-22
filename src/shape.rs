use crate::point::GeographicPoint;

/// Represents one of both shape kinds: close or open.
#[derive(Debug, Default, Clone, Copy)]
pub enum Kind {
    #[default]
    Open,
    Close,
}

/// Represents a combination of [`GeographicPoint`]s tracing a shape.
#[derive(Debug, Default, Clone)]
pub struct GeographicShape {
    points: Vec<GeographicPoint>,
    kind: Kind,
}

impl GeographicShape {
    pub fn new(kind: Kind, points: &[GeographicPoint]) -> Self {
        Self {
            points: points.into(),
            kind,
        }
    }

    pub fn points(&self) -> &[GeographicPoint] {
        &self.points
    }

    pub fn points_mut(&mut self) -> &mut [GeographicPoint] {
        &mut self.points
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }
}
