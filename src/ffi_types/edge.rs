#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Edge {
	Left = 0,
	Top = 1,
	Right = 2,
	Bottom = 3,
	Start = 4,
	End = 5,
	Horizontal = 6,
	Vertical = 7,
	All = 8,
}
