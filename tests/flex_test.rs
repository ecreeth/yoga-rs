/**
 * Copyright (c) 2014-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// @Generated by gentest/gentest.rb from gentest/fixtures/YGFlexTest.html

extern crate ordered_float;
extern crate yoga;

use yoga::*;

#[test]
fn test_flex_basis_flex_grow_column() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_flex_basis(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(75 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(75 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(25 as f32, root_child1.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(75 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(75 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(25 as f32, root_child1.get_layout_height());
}

#[test]
fn test_flex_basis_flex_grow_row() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_flex_basis(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(75 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(75 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(25 as f32, root_child1.get_layout_width());
	assert_eq!(100 as f32, root_child1.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(75 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(25 as f32, root_child1.get_layout_width());
	assert_eq!(100 as f32, root_child1.get_layout_height());
}

#[test]
fn test_flex_basis_flex_shrink_column() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_shrink(1 as f32 as f32);
	root_child0.set_flex_basis(StyleUnit::Point((100 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_basis(StyleUnit::Point((50 as f32).into()));
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(50 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(50 as f32, root_child1.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(50 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(50 as f32, root_child1.get_layout_height());
}

#[test]
fn test_flex_basis_flex_shrink_row() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_shrink(1 as f32 as f32);
	root_child0.set_flex_basis(StyleUnit::Point((100 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_basis(StyleUnit::Point((50 as f32).into()));
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(50 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(100 as f32, root_child1.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(50 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(100 as f32, root_child1.get_layout_height());
}

#[test]
fn test_flex_shrink_to_zero() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_height(StyleUnit::Point((75 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_width(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_shrink(1 as f32 as f32);
	root_child1.set_width(StyleUnit::Point((50 as f32).into()));
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_height(StyleUnit::Point((50 as f32).into()));
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new_with_config(&mut config);
	root_child2.set_width(StyleUnit::Point((50 as f32).into()));
	root_child2.set_min_width(StyleUnit::Auto);
	root_child2.set_height(StyleUnit::Point((50 as f32).into()));
	root_child2.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(50 as f32, root.get_layout_width());
	assert_eq!(75 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(50 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(50 as f32, root_child2.get_layout_top());
	assert_eq!(50 as f32, root_child2.get_layout_width());
	assert_eq!(50 as f32, root_child2.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(50 as f32, root.get_layout_width());
	assert_eq!(75 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(50 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(50 as f32, root_child2.get_layout_top());
	assert_eq!(50 as f32, root_child2.get_layout_width());
	assert_eq!(50 as f32, root_child2.get_layout_height());
}

#[test]
fn test_flex_basis_overrides_main_size() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_flex_basis(StyleUnit::Point((50 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((20 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_height(StyleUnit::Point((10 as f32).into()));
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new_with_config(&mut config);
	root_child2.set_flex_grow(1 as f32);
	root_child2.set_min_width(StyleUnit::Auto);
	root_child2.set_height(StyleUnit::Point((10 as f32).into()));
	root_child2.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(60 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(60 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(20 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(80 as f32, root_child2.get_layout_top());
	assert_eq!(100 as f32, root_child2.get_layout_width());
	assert_eq!(20 as f32, root_child2.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(60 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(60 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(20 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(80 as f32, root_child2.get_layout_top());
	assert_eq!(100 as f32, root_child2.get_layout_width());
	assert_eq!(20 as f32, root_child2.get_layout_height());
}

#[test]
fn test_flex_grow_shrink_at_most() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new_with_config(&mut config);
	root_child0_child0.set_flex_grow(1 as f32);
	root_child0_child0.set_flex_shrink(1 as f32 as f32);
	root_child0_child0.set_min_width(StyleUnit::Auto);
	root_child0_child0.set_min_height(StyleUnit::Auto);
	root_child0.insert_child(&mut root_child0_child0, 0);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(0 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child0_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0_child0.get_layout_width());
	assert_eq!(0 as f32, root_child0_child0.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(0 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child0_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0_child0.get_layout_width());
	assert_eq!(0 as f32, root_child0_child0.get_layout_height());
}

#[test]
fn test_flex_grow_less_than_factor_one() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((200 as f32).into()));
	root.set_height(StyleUnit::Point((500 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(0.2 as f32);
	root_child0.set_flex_basis(StyleUnit::Point((40 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(0.2 as f32);
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new_with_config(&mut config);
	root_child2.set_flex_grow(0.4 as f32);
	root_child2.set_min_width(StyleUnit::Auto);
	root_child2.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(200 as f32, root.get_layout_width());
	assert_eq!(500 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(200 as f32, root_child0.get_layout_width());
	assert_eq!(132 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(132 as f32, root_child1.get_layout_top());
	assert_eq!(200 as f32, root_child1.get_layout_width());
	assert_eq!(92 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(224 as f32, root_child2.get_layout_top());
	assert_eq!(200 as f32, root_child2.get_layout_width());
	assert_eq!(184 as f32, root_child2.get_layout_height());

	root.calculate_layout(UNDEFINED, UNDEFINED, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(200 as f32, root.get_layout_width());
	assert_eq!(500 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(200 as f32, root_child0.get_layout_width());
	assert_eq!(132 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(132 as f32, root_child1.get_layout_top());
	assert_eq!(200 as f32, root_child1.get_layout_width());
	assert_eq!(92 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(224 as f32, root_child2.get_layout_top());
	assert_eq!(200 as f32, root_child2.get_layout_width());
	assert_eq!(184 as f32, root_child2.get_layout_height());
}
