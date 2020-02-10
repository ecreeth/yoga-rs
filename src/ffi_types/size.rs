#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Size {
	pub width: f32,
	pub height: f32,
}
