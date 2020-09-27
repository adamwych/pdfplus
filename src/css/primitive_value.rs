#![allow(unused)]

use crate::utils::Color;

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveValueKind {
    Identifier,
    Color,
    String,
    DimensionValue,
    None
}

#[derive(Debug, Clone)]
pub struct PrimitiveValue {
    pub kind: PrimitiveValueKind,
    pub value: String,
    color: Color,
    dimension_value: DimensionValue
}

#[derive(Debug, Default, Clone)]
pub struct DimensionValue {
    pub value: f64,
    pub dimension: String
}

impl PrimitiveValue {
    pub fn as_color(&self) -> &Color { &self.color }
    pub fn as_string(&self) -> &String { &self.value }
    pub fn as_dimension_value(&self) -> &DimensionValue { &self.dimension_value }
    
    pub fn is_color(&self) -> bool { self.kind == PrimitiveValueKind::Color }
    pub fn is_string(&self) -> bool { self.kind == PrimitiveValueKind::String }
    pub fn is_dimension_value(&self) -> bool { self.kind == PrimitiveValueKind::DimensionValue }
    pub fn is_identifier(&self) -> bool { self.kind == PrimitiveValueKind::Identifier }
    pub fn is_none(&self) -> bool { self.kind == PrimitiveValueKind::None }

    pub fn has_value(&self) -> bool { !self.value.is_empty() }
}

impl PrimitiveValue {
    pub fn from_color(value: &String, color: Color) -> Self {
        Self {
            kind: PrimitiveValueKind::Color,
            value: value.clone(),
            color: color,
            ..Self::default()
        }
    }

    pub fn from_string(value: &String) -> Self {
        Self {
            kind: PrimitiveValueKind::String,
            value: value.clone(),
            ..Self::default()
        }
    }
    
    pub fn from_dimension_value(full_value: &String, value: f64, dimension: &String) -> Self {
        Self {
            kind: PrimitiveValueKind::String,
            value: full_value.clone(),
            dimension_value: DimensionValue {
                value: value,
                dimension: dimension.clone()
            },
            ..Self::default()
        }
    }

    pub fn from_identifier(value: &String) -> Self {
        Self {
            kind: PrimitiveValueKind::Identifier,
            value: value.clone(),
            ..Self::default()
        }
    }

    pub fn default() -> Self {
        Self {
            kind: PrimitiveValueKind::None,
            value: String::new(),
            color: Color::default(),
            dimension_value: DimensionValue::default()
        }
    }
}