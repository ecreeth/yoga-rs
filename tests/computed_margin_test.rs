#[macro_use]
extern crate yoga;

use yoga::{Direction, Node};
use yoga::prelude::*;

#[test]
fn test_computed_layout_margin() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt),
		MarginStart(10 %)
	);

	root.calculate_layout(100.0, 100.0, Direction::LTR);

	assert_eq!(10.0, root.get_layout_margin_left());
	assert_eq!(0.0, root.get_layout_margin_right());

	root.calculate_layout(100.0, 100.0, Direction::RTL);

	assert_eq!(0.0, root.get_layout_margin_left());
	assert_eq!(10.0, root.get_layout_margin_right());
}
