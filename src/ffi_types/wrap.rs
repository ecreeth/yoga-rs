#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Wrap {
	NoWrap = 0,
	Wrap = 1,
	WrapReverse = 2,
}
