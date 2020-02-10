/**
 * Copyright (c) 2014-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// @Generated by gentest/gentest.rb from gentest/fixtures/YGAlignSelfTest.html

extern crate ordered_float;
extern crate yoga;

use yoga::*;

#[test]
fn test_align_self_center() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::Center);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(45 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(45 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_align_self_flex_end() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::FlexEnd);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(90 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_align_self_flex_start() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::FlexStart);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(90 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_align_self_flex_end_override_flex_start() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_align_items(Align::FlexStart);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::FlexEnd);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(90 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_align_self_baseline() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::Baseline);
	root_child0.set_width(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_align_self(Align::Baseline);
	root_child1.set_width(StyleUnit::Point((50 as f32).into()));
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_height(StyleUnit::Point((20 as f32).into()));
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);

	let mut root_child1_child0 = Node::new_with_config(&mut config);
	root_child1_child0.set_width(StyleUnit::Point((50 as f32).into()));
	root_child1_child0.set_min_width(StyleUnit::Auto);
	root_child1_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child1_child0.set_min_height(StyleUnit::Auto);
	root_child1.insert_child(&mut root_child1_child0, 0);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(50 as f32, root_child1.get_layout_left());
	assert_eq!(40 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(20 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child1_child0.get_layout_left());
	assert_eq!(0 as f32, root_child1_child0.get_layout_top());
	assert_eq!(50 as f32, root_child1_child0.get_layout_width());
	assert_eq!(10 as f32, root_child1_child0.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(50 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(40 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(20 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child1_child0.get_layout_left());
	assert_eq!(0 as f32, root_child1_child0.get_layout_top());
	assert_eq!(50 as f32, root_child1_child0.get_layout_width());
	assert_eq!(10 as f32, root_child1_child0.get_layout_height());
}
