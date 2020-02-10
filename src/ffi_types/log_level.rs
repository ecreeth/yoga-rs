#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LogLevel {
	Error = 0,
	Warn = 1,
	Info = 2,
	Debug = 3,
	Verbose = 4,
	Fatal = 5,
}
