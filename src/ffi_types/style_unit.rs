use crate::internal::{YGUnit, YGValue};
use ordered_float::OrderedFloat;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum StyleUnit {
	UndefinedValue,
	Point(OrderedFloat<f32>),
	Percent(OrderedFloat<f32>),
	Auto,
}
impl From<StyleUnit> for YGUnit {
	fn from(s: StyleUnit) -> YGUnit {
		match s {
			StyleUnit::Auto 					=> YGUnit::YGUnitAuto,
			StyleUnit::Point(_) 			=> YGUnit::YGUnitPoint,
			StyleUnit::Percent(_) 		=> YGUnit::YGUnitPercent,
			StyleUnit::UndefinedValue => YGUnit::YGUnitUndefined,
		}
	}
}

impl From<YGValue> for StyleUnit {
	fn from(v: YGValue) -> StyleUnit {
		match v.unit {
			YGUnit::YGUnitAuto 			=> StyleUnit::Auto,
			YGUnit::YGUnitUndefined => StyleUnit::UndefinedValue,
			YGUnit::YGUnitPoint 		=> StyleUnit::Point(OrderedFloat(v.value)),
			YGUnit::YGUnitPercent 	=> StyleUnit::Percent(OrderedFloat(v.value)),
		}
	}
}
