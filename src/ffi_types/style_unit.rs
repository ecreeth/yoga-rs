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
