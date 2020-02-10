#[allow(unused_imports)]

#[cfg(feature = "serde_support")]
use serde;
#[macro_use]
#[cfg(feature = "serde_support")]
use serde_derive;

mod internal {

	#![allow(dead_code)]

	use std::os::raw::c_char;

	use crate::ffi_types::align::*;
	
	use crate::ffi_types::dimension::*;
	use crate::ffi_types::direction::*;
	use crate::ffi_types::display::*;
	use crate::ffi_types::edge::*;
	use crate::ffi_types::flex_direction::*;
	use crate::ffi_types::justify::*;
	use crate::ffi_types::log_level::*;
	use crate::ffi_types::measure_mode::*;
	// use crate::ffi_types::node_ref::*;
	use crate::ffi_types::node_type::*;
	use crate::ffi_types::overflow::*;
	use crate::ffi_types::position_type::*;
	use crate::ffi_types::print_options::*;
	use crate::ffi_types::size::*;
	use crate::ffi_types::style_unit::*;
	
	use crate::ffi_types::wrap::*;

	#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
	pub const YGUndefined: ::std::os::raw::c_double = 1000000000000000000000.0;
	pub const YGAlignCount: u32 = 8;
	pub const YGDimensionCount: u32 = 2;
	pub const YGDirectionCount: u32 = 3;
	pub const YGDisplayCount: u32 = 2;
	pub const YGEdgeCount: u32 = 9;
	pub const YGExperimentalFeatureCount: u32 = 1;
	pub const YGFlexDirectionCount: u32 = 4;
	pub const YGJustifyCount: u32 = 6;
	pub const YGLogLevelCount: u32 = 6;
	pub const YGMeasureModeCount: u32 = 3;
	pub const YGNodeTypeCount: u32 = 2;
	pub const YGOverflowCount: u32 = 3;
	pub const YGPositionTypeCount: u32 = 2;
	pub const YGPrintOptionsCount: u32 = 3;
	pub const YGUnitCount: u32 = 4;
	pub const YGWrapCount: u32 = 3;
	pub type Int32T = ::std::os::raw::c_int;
	pub type Uint32T = ::std::os::raw::c_uint;
	pub type VaList = BuiltinVaList;

	type TChar = *const c_char;
	type TFloat = ::std::os::raw::c_float;

	extern "C" {
		pub fn YGAlignToString(value: Align) -> TChar;
	}
	extern "C" {
		pub fn YGDimensionToString(value: Dimension) -> TChar;
	}
	extern "C" {
		pub fn YGDirectionToString(value: Direction) -> TChar;
	}
	extern "C" {
		pub fn YGDisplayToString(value: Display) -> TChar;
	}
	extern "C" {
		pub fn YGEdgeToString(value: Edge) -> TChar;
	}
	#[repr(u32)]
	#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
	pub enum YGExperimentalFeature {
		YGExperimentalFeatureWebFlexBasis = 0,
	}
	extern "C" {
		pub fn YGExperimentalFeatureToString(
			value: YGExperimentalFeature,
		) -> TChar;
	}
	extern "C" {
		pub fn YGFlexDirectionToString(
			value: FlexDirection,
		) -> TChar;
	}
	extern "C" {
		pub fn YGJustifyToString(value: Justify) -> TChar;
	}
	extern "C" {
		pub fn YGLogLevelToString(value: LogLevel) -> TChar;
	}
	extern "C" {
		pub fn YGMeasureModeToString(value: MeasureMode) -> TChar;
	}
	extern "C" {
		pub fn YGNodeTypeToString(value: NodeType) -> TChar;
	}
	extern "C" {
		pub fn YGOverflowToString(value: Overflow) -> TChar;
	}
	extern "C" {
		pub fn YGPositionTypeToString(value: PositionType)
			-> TChar;
	}
	extern "C" {
		pub fn YGPrintOptionsToString(value: PrintOptions)
			-> TChar;
	}
	extern "C" {
		pub fn YGUnitToString(value: StyleUnit) -> TChar;
	}
	extern "C" {
		pub fn YGWrapToString(value: Wrap) -> TChar;
	}
	#[repr(C)]
	#[derive(Debug, Copy, Clone)]
	pub struct YGValue {
		pub value: TFloat,
		pub unit: StyleUnit,
	}
	extern "C" {
		pub static YGValueUndefined: YGValue;
	}
	extern "C" {
		pub static YGValueAuto: YGValue;
	}
	#[repr(C)]
	#[derive(Debug, Copy, Clone)]
	pub struct YGConfig {
		_unused: [u8; 0],
	}
	pub type YGConfigRef = *mut YGConfig;
	#[repr(C)]
	#[derive(Debug, Copy, Clone)]
	pub struct YGNode {
		_unused: [u8; 0],
	}
	pub type NodeRef = *mut YGNode;
	pub type YGMeasureFunc = ::std::option::Option<
		unsafe extern "C" fn(
			node: NodeRef,
			width: TFloat,
			widthMode: MeasureMode,
			height: TFloat,
			heightMode: MeasureMode,
		) -> Size,
	>;
	pub type YGBaselineFunc = ::std::option::Option<
		unsafe extern "C" fn(
			node: NodeRef,
			width: TFloat,
			height: TFloat,
		) -> TFloat,
	>;
	pub type YGDirtiedFunc = ::std::option::Option<unsafe extern "C" fn(node: NodeRef)>;
	pub type YGPrintFunc = ::std::option::Option<unsafe extern "C" fn(node: NodeRef)>;
	pub type YGLogger = ::std::option::Option<
		unsafe extern "C" fn(
			config: YGConfigRef,
			node: NodeRef,
			level: LogLevel,
			format: TChar,
			args: *mut __va_list_tag,
		) -> ::std::os::raw::c_int,
	>;
	pub type YGCloneNodeFunc = ::std::option::Option<
		unsafe extern "C" fn(
			oldNode: NodeRef,
			owner: NodeRef,
			childIndex: ::std::os::raw::c_int,
		) -> NodeRef,
	>;
	extern "C" {
		pub fn YGNodeNew() -> NodeRef;
	}
	extern "C" {
		pub fn YGNodeNewWithConfig(config: YGConfigRef) -> NodeRef;
	}
	extern "C" {
		pub fn YGNodeClone(node: NodeRef) -> NodeRef;
	}
	extern "C" {
		pub fn YGNodeFree(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeFreeRecursive(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeReset(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeGetInstanceCount() -> i32;
	}
	extern "C" {
		pub fn YGNodeInsertChild(node: NodeRef, child: NodeRef, index: u32);
	}
	extern "C" {
		pub fn YGNodeInsertSharedChild(node: NodeRef, child: NodeRef, index: u32);
	}
	extern "C" {
		pub fn YGNodeRemoveChild(node: NodeRef, child: NodeRef);
	}
	extern "C" {
		pub fn YGNodeRemoveAllChildren(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeGetChild(node: NodeRef, index: u32) -> NodeRef;
	}
	extern "C" {
		pub fn YGNodeGetOwner(node: NodeRef) -> NodeRef;
	}
	extern "C" {
		pub fn YGNodeGetParent(node: NodeRef) -> NodeRef;
	}
	extern "C" {
		pub fn YGNodeGetChildCount(node: NodeRef) -> u32;
	}
	extern "C" {
		pub fn YGNodeSetChildren(
			owner: NodeRef,
			children: *const NodeRef,
			count: u32,
		);
	}
	extern "C" {
		pub fn YGNodeCalculateLayout(
			node: NodeRef,
			availableWidth: TFloat,
			availableHeight: TFloat,
			ownerDirection: Direction,
		);
	}
	extern "C" {
		pub fn YGNodeMarkDirty(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeMarkDirtyAndPropogateToDescendants(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodePrint(node: NodeRef, options: PrintOptions);
	}
	extern "C" {
		pub fn YGFloatIsUndefined(value: TFloat) -> bool;
	}
	extern "C" {
		pub fn YGNodeCanUseCachedMeasurement(
			widthMode: MeasureMode,
			width: TFloat,
			heightMode: MeasureMode,
			height: TFloat,
			lastWidthMode: MeasureMode,
			lastWidth: TFloat,
			lastHeightMode: MeasureMode,
			lastHeight: TFloat,
			lastComputedWidth: TFloat,
			lastComputedHeight: TFloat,
			marginRow: TFloat,
			marginColumn: TFloat,
			config: YGConfigRef,
		) -> bool;
	}
	extern "C" {
		pub fn YGNodeCopyStyle(dstNode: NodeRef, srcNode: NodeRef);
	}
	extern "C" {
		pub fn YGNodeGetContext(node: NodeRef) -> *mut ::std::os::raw::c_void;
	}
	extern "C" {
		pub fn YGNodeSetContext(node: NodeRef, context: *mut ::std::os::raw::c_void);
	}
	extern "C" {
		pub fn YGConfigSetPrintTreeFlag(config: YGConfigRef, enabled: bool);
	}
	extern "C" {
		pub fn YGNodeGetMeasureFunc(node: NodeRef) -> YGMeasureFunc;
	}
	extern "C" {
		pub fn YGNodeSetMeasureFunc(node: NodeRef, measureFunc: YGMeasureFunc);
	}
	extern "C" {
		pub fn YGNodeGetBaselineFunc(node: NodeRef) -> YGBaselineFunc;
	}
	extern "C" {
		pub fn YGNodeSetBaselineFunc(node: NodeRef, baselineFunc: YGBaselineFunc);
	}
	extern "C" {
		pub fn YGNodeGetDirtiedFunc(node: NodeRef) -> YGDirtiedFunc;
	}
	extern "C" {
		pub fn YGNodeSetDirtiedFunc(node: NodeRef, dirtiedFunc: YGDirtiedFunc);
	}
	extern "C" {
		pub fn YGNodeGetPrintFunc(node: NodeRef) -> YGPrintFunc;
	}
	extern "C" {
		pub fn YGNodeSetPrintFunc(node: NodeRef, printFunc: YGPrintFunc);
	}
	extern "C" {
		pub fn YGNodeGetHasNewLayout(node: NodeRef) -> bool;
	}
	extern "C" {
		pub fn YGNodeSetHasNewLayout(node: NodeRef, hasNewLayout: bool);
	}
	extern "C" {
		pub fn YGNodeGetNodeType(node: NodeRef) -> NodeType;
	}
	extern "C" {
		pub fn YGNodeSetNodeType(node: NodeRef, nodeType: NodeType);
	}
	extern "C" {
		pub fn YGNodeIsDirty(node: NodeRef) -> bool;
	}
	extern "C" {
		pub fn YGNodeLayoutGetDidUseLegacyFlag(node: NodeRef) -> bool;
	}
	extern "C" {
		pub fn YGNodeStyleSetDirection(node: NodeRef, direction: Direction);
	}
	extern "C" {
		pub fn YGNodeStyleGetDirection(node: NodeRef) -> Direction;
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexDirection(
			node: NodeRef,
			flexDirection: FlexDirection,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetFlexDirection(node: NodeRef) -> FlexDirection;
	}
	extern "C" {
		pub fn YGNodeStyleSetJustifyContent(node: NodeRef, justifyContent: Justify);
	}
	extern "C" {
		pub fn YGNodeStyleGetJustifyContent(node: NodeRef) -> Justify;
	}
	extern "C" {
		pub fn YGNodeStyleSetAlignContent(node: NodeRef, alignContent: Align);
	}
	extern "C" {
		pub fn YGNodeStyleGetAlignContent(node: NodeRef) -> Align;
	}
	extern "C" {
		pub fn YGNodeStyleSetAlignItems(node: NodeRef, alignItems: Align);
	}
	extern "C" {
		pub fn YGNodeStyleGetAlignItems(node: NodeRef) -> Align;
	}
	extern "C" {
		pub fn YGNodeStyleSetAlignSelf(node: NodeRef, alignSelf: Align);
	}
	extern "C" {
		pub fn YGNodeStyleGetAlignSelf(node: NodeRef) -> Align;
	}
	extern "C" {
		pub fn YGNodeStyleSetPositionType(
			node: NodeRef,
			positionType: PositionType,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetPositionType(node: NodeRef) -> PositionType;
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexWrap(node: NodeRef, flexWrap: Wrap);
	}
	extern "C" {
		pub fn YGNodeStyleGetFlexWrap(node: NodeRef) -> Wrap;
	}
	extern "C" {
		pub fn YGNodeStyleSetOverflow(node: NodeRef, overflow: Overflow);
	}
	extern "C" {
		pub fn YGNodeStyleGetOverflow(node: NodeRef) -> Overflow;
	}
	extern "C" {
		pub fn YGNodeStyleSetDisplay(node: NodeRef, display: Display);
	}
	extern "C" {
		pub fn YGNodeStyleGetDisplay(node: NodeRef) -> Display;
	}
	extern "C" {
		pub fn YGNodeStyleSetFlex(node: NodeRef, flex: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleGetFlex(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexGrow(node: NodeRef, flexGrow: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleGetFlexGrow(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexShrink(node: NodeRef, flexShrink: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleGetFlexShrink(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexBasis(node: NodeRef, flexBasis: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexBasisPercent(
			node: NodeRef,
			flexBasis: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleSetFlexBasisAuto(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeStyleGetFlexBasis(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetPosition(
			node: NodeRef,
			edge: Edge,
			position: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleSetPositionPercent(
			node: NodeRef,
			edge: Edge,
			position: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetPosition(node: NodeRef, edge: Edge) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetMargin(
			node: NodeRef,
			edge: Edge,
			margin: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleSetMarginPercent(
			node: NodeRef,
			edge: Edge,
			margin: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleSetMarginAuto(node: NodeRef, edge: Edge);
	}
	extern "C" {
		pub fn YGNodeStyleGetMargin(node: NodeRef, edge: Edge) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetPadding(
			node: NodeRef,
			edge: Edge,
			padding: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleSetPaddingPercent(
			node: NodeRef,
			edge: Edge,
			padding: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetPadding(node: NodeRef, edge: Edge) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetBorder(
			node: NodeRef,
			edge: Edge,
			border: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetBorder(
			node: NodeRef,
			edge: Edge,
		) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeStyleSetWidth(node: NodeRef, width: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetWidthPercent(node: NodeRef, width: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetWidthAuto(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeStyleGetWidth(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetHeight(node: NodeRef, height: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetHeightPercent(node: NodeRef, height: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetHeightAuto(node: NodeRef);
	}
	extern "C" {
		pub fn YGNodeStyleGetHeight(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetMinWidth(node: NodeRef, minWidth: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetMinWidthPercent(
			node: NodeRef,
			minWidth: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetMinWidth(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetMinHeight(node: NodeRef, minHeight: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetMinHeightPercent(
			node: NodeRef,
			minHeight: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetMinHeight(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetMaxWidth(node: NodeRef, maxWidth: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetMaxWidthPercent(
			node: NodeRef,
			maxWidth: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetMaxWidth(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetMaxHeight(node: NodeRef, maxHeight: TFloat);
	}
	extern "C" {
		pub fn YGNodeStyleSetMaxHeightPercent(
			node: NodeRef,
			maxHeight: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetMaxHeight(node: NodeRef) -> YGValue;
	}
	extern "C" {
		pub fn YGNodeStyleSetAspectRatio(
			node: NodeRef,
			aspectRatio: TFloat,
		);
	}
	extern "C" {
		pub fn YGNodeStyleGetAspectRatio(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetLeft(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetTop(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetRight(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetBottom(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetWidth(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetHeight(node: NodeRef) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetDirection(node: NodeRef) -> Direction;
	}
	extern "C" {
		pub fn YGNodeLayoutGetHadOverflow(node: NodeRef) -> bool;
	}
	extern "C" {
		pub fn YGNodeLayoutGetDidLegacyStretchFlagAffectLayout(node: NodeRef) -> bool;
	}
	extern "C" {
		pub fn YGNodeLayoutGetMargin(
			node: NodeRef,
			edge: Edge,
		) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetBorder(
			node: NodeRef,
			edge: Edge,
		) -> TFloat;
	}
	extern "C" {
		pub fn YGNodeLayoutGetPadding(
			node: NodeRef,
			edge: Edge,
		) -> TFloat;
	}
	extern "C" {
		pub fn YGConfigSetLogger(config: YGConfigRef, logger: YGLogger);
	}
	extern "C" {
		pub fn YGLog(
			node: NodeRef,
			level: LogLevel,
			message: TChar,
			...
		);
	}
	extern "C" {
		pub fn YGLogWithConfig(
			config: YGConfigRef,
			level: LogLevel,
			format: TChar,
			...
		);
	}
	extern "C" {
		pub fn YGAssert(condition: bool, message: TChar);
	}
	extern "C" {
		pub fn YGAssertWithNode(
			node: NodeRef,
			condition: bool,
			message: TChar,
		);
	}
	extern "C" {
		pub fn YGAssertWithConfig(
			config: YGConfigRef,
			condition: bool,
			message: TChar,
		);
	}
	extern "C" {
		pub fn YGConfigSetPointScaleFactor(
			config: YGConfigRef,
			pixelsInPoint: TFloat,
		);
	}
	extern "C" {
		pub fn YGConfigSetShouldDiffLayoutWithoutLegacyStretchBehaviour(
			config: YGConfigRef,
			shouldDiffLayout: bool,
		);
	}
	extern "C" {
		pub fn YGConfigSetUseLegacyStretchBehaviour(
			config: YGConfigRef,
			useLegacyStretchBehaviour: bool,
		);
	}
	extern "C" {
		pub fn YGConfigNew() -> YGConfigRef;
	}
	extern "C" {
		pub fn YGConfigFree(config: YGConfigRef);
	}
	extern "C" {
		pub fn YGConfigCopy(dest: YGConfigRef, src: YGConfigRef);
	}
	extern "C" {
		pub fn YGConfigGetInstanceCount() -> i32;
	}
	extern "C" {
		pub fn YGConfigSetExperimentalFeatureEnabled(
			config: YGConfigRef,
			feature: YGExperimentalFeature,
			enabled: bool,
		);
	}
	extern "C" {
		pub fn YGConfigIsExperimentalFeatureEnabled(
			config: YGConfigRef,
			feature: YGExperimentalFeature,
		) -> bool;
	}
	extern "C" {
		pub fn YGConfigSetUseWebDefaults(config: YGConfigRef, enabled: bool);
	}
	extern "C" {
		pub fn YGConfigGetUseWebDefaults(config: YGConfigRef) -> bool;
	}
	extern "C" {
		pub fn YGConfigSetCloneNodeFunc(config: YGConfigRef, callback: YGCloneNodeFunc);
	}
	extern "C" {
		pub fn YGConfigGetDefault() -> YGConfigRef;
	}
	extern "C" {
		pub fn YGConfigSetContext(config: YGConfigRef, context: *mut ::std::os::raw::c_void);
	}
	extern "C" {
		pub fn YGConfigGetContext(config: YGConfigRef) -> *mut ::std::os::raw::c_void;
	}
	extern "C" {
		pub fn YGRoundValueToPixelGrid(
			value: TFloat,
			pointScaleFactor: TFloat,
			forceCeil: bool,
			forceFloor: bool,
		) -> TFloat;
	}
	pub type BuiltinVaList = [__va_list_tag; 1usize];
	#[repr(C)]
	#[derive(Debug, Copy, Clone)]
	pub struct __va_list_tag {
		pub gp_offset: ::std::os::raw::c_uint,
		pub fp_offset: ::std::os::raw::c_uint,
		pub overflow_arg_area: *mut ::std::os::raw::c_void,
		pub reg_save_area: *mut ::std::os::raw::c_void,
	}
}

// Public re-exports of Yoga enums
mod ffi_types {
	pub mod align;
	pub mod config_ref;
	pub mod dimension;
	pub mod direction;
	pub mod display;
	pub mod edge;
	pub mod flex_direction;
	pub mod justify;
	pub mod log_level;
	pub mod measure_mode;
	pub mod node_ref;
	pub mod node_type;
	pub mod overflow;
	pub mod position_type;
	pub mod print_options;
	pub mod size;
	pub mod style_unit;
	pub mod undefined;
	pub mod wrap;
}

pub mod prelude;
pub mod traits;
pub mod types;

use std::any::Any;
pub use crate::types::*;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
	inner_config: ConfigRef,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
	inner_node: NodeRef,
}

impl Config {
	pub fn new() -> Config {
		Config {
			inner_config: unsafe { internal::YGConfigNew() },
		}
	}
}

impl Node {
	pub fn new() -> Node {
		Node {
			inner_node: unsafe { internal::YGNodeNew() },
		}
	}

	pub fn new_with_config(config: &mut Config) -> Node {
		Node {
			inner_node: unsafe { internal::YGNodeNewWithConfig(config.inner_config) },
		}
	}

	pub fn reset(&mut self) {
		unsafe {
			internal::YGNodeReset(self.inner_node);
		}
	}

	pub fn mark_dirty(&mut self) {
		unsafe {
			internal::YGNodeMarkDirty(self.inner_node);
		}
	}

	pub fn apply_styles<'a, I>(&mut self, styles: I)
	where
		I: IntoIterator<Item = &'a FlexStyle>,
	{
		for style in styles {
			self.apply_style(style);
		}
	}

	pub fn apply_style(&mut self, style: &FlexStyle) {
		use FlexStyle::*;

		match *style {
			AlignContent(align) => self.set_align_content(align),
			AlignItems(align) => self.set_align_items(align),
			AlignSelf(align) => self.set_align_self(align),
			AspectRatio(a) => self.set_aspect_ratio(a.into_inner()),
			BorderBottom(b) => self.set_border(Edge::Bottom, b.into_inner()),
			BorderEnd(b) => self.set_border(Edge::End, b.into_inner()),
			BorderLeft(b) => self.set_border(Edge::Left, b.into_inner()),
			BorderRight(b) => self.set_border(Edge::Right, b.into_inner()),
			BorderStart(b) => self.set_border(Edge::Start, b.into_inner()),
			BorderTop(b) => self.set_border(Edge::Top, b.into_inner()),
			Border(b) => self.set_border(Edge::All, b.into_inner()),
			Bottom(b) => self.set_position(Edge::Bottom, b),
			Display(d) => self.set_display(d),
			End(e) => self.set_position(Edge::End, e),
			Flex(f) => self.set_flex(f.into_inner()),
			FlexBasis(f) => self.set_flex_basis(f),
			FlexGrow(f) => self.set_flex_grow(f.into_inner()),
			FlexDirection(flex_direction) => self.set_flex_direction(flex_direction),
			FlexShrink(f) => self.set_flex_shrink(f.into_inner()),
			FlexWrap(wrap) => self.set_flex_wrap(wrap),
			Height(h) => self.set_height(h),
			JustifyContent(justify) => self.set_justify_content(justify),
			Left(l) => self.set_position(Edge::Left, l),
			Margin(m) => self.set_margin(Edge::All, m),
			MarginBottom(m) => self.set_margin(Edge::Bottom, m),
			MarginEnd(m) => self.set_margin(Edge::End, m),
			MarginHorizontal(m) => self.set_margin(Edge::Horizontal, m),
			MarginLeft(m) => self.set_margin(Edge::Left, m),
			MarginRight(m) => self.set_margin(Edge::Right, m),
			MarginStart(m) => self.set_margin(Edge::Start, m),
			MarginTop(m) => self.set_margin(Edge::Top, m),
			MarginVertical(m) => self.set_margin(Edge::Vertical, m),
			MaxHeight(h) => self.set_max_height(h),
			MaxWidth(w) => self.set_max_width(w),
			MinHeight(h) => self.set_min_height(h),
			MinWidth(w) => self.set_min_width(w),
			Overflow(o) => self.set_overflow(o),
			Padding(p) => self.set_padding(Edge::All, p),
			PaddingBottom(p) => self.set_padding(Edge::Bottom, p),
			PaddingEnd(p) => self.set_padding(Edge::End, p),
			PaddingHorizontal(p) => self.set_padding(Edge::Horizontal, p),
			PaddingLeft(p) => self.set_padding(Edge::Left, p),
			PaddingRight(p) => self.set_padding(Edge::Right, p),
			PaddingStart(p) => self.set_padding(Edge::Start, p),
			PaddingTop(p) => self.set_padding(Edge::Top, p),
			PaddingVertical(p) => self.set_padding(Edge::Vertical, p),
			Position(position_type) => self.set_position_type(position_type),
			Right(r) => self.set_position(Edge::Right, r),
			Start(s) => self.set_position(Edge::Start, s),
			Top(t) => self.set_position(Edge::Top, t),
			Width(w) => self.set_width(w),
		}
	}

	pub fn insert_child(&mut self, child: &mut Node, index: u32) {
		unsafe {
			internal::YGNodeInsertChild(self.inner_node, child.inner_node, index);
		}
	}

	pub fn remove_child(&mut self, child: &mut Node) {
		unsafe {
			internal::YGNodeRemoveChild(self.inner_node, child.inner_node);
		}
	}

	pub fn child_count(&self) -> u32 {
		unsafe { internal::YGNodeGetChildCount(self.inner_node) }
	}

	pub fn set_direction(&mut self, direction: Direction) {
		unsafe {
			internal::YGNodeStyleSetDirection(
				self.inner_node,
				ffi_types::direction::Direction::from(direction),
			);
		}
	}

	pub fn set_flex_direction(&mut self, direction: FlexDirection) {
		unsafe {
			internal::YGNodeStyleSetFlexDirection(
				self.inner_node,
				FlexDirection::from(direction),
			);
		}
	}

	pub fn set_justify_content(&mut self, justify: Justify) {
		unsafe {
			internal::YGNodeStyleSetJustifyContent(
				self.inner_node,
				Justify::from(justify),
			);
		}
	}

	pub fn set_align_content(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignContent(self.inner_node, Align::from(align));
		}
	}

	pub fn set_align_items(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignItems(self.inner_node, Align::from(align));
		}
	}

	pub fn set_align_self(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignSelf(self.inner_node, Align::from(align));
		}
	}

	pub fn set_position_type(&mut self, position_type: PositionType) {
		unsafe {
			internal::YGNodeStyleSetPositionType(
				self.inner_node,
				PositionType::from(position_type),
			);
		}
	}

	pub fn set_position(&mut self, edge: Edge, position: StyleUnit) {
		unsafe {
			match position {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetPosition(
					self.inner_node,
					Edge::from(edge),
					UNDEFINED,
				),
				StyleUnit::Point(val) => internal::YGNodeStyleSetPosition(
					self.inner_node,
					Edge::from(edge),
					val.into_inner(),
				),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetPositionPercent(
					self.inner_node,
					Edge::from(edge),
					val.into_inner(),
				),
				// auto is not a valid value for position
				StyleUnit::Auto => internal::YGNodeStyleSetPosition(
					self.inner_node,
					Edge::from(edge),
					UNDEFINED,
				),
			}
		}
	}

	pub fn set_flex_wrap(&mut self, wrap: Wrap) {
		unsafe {
			internal::YGNodeStyleSetFlexWrap(self.inner_node, Wrap::from(wrap));
		}
	}

	pub fn set_overflow(&mut self, overflow: Overflow) {
		unsafe {
			internal::YGNodeStyleSetOverflow(self.inner_node, Overflow::from(overflow));
		}
	}

	pub fn set_flex(&mut self, flex: f32) {
		unsafe {
			internal::YGNodeStyleSetFlex(self.inner_node, flex);
		}
	}

	pub fn set_flex_grow(&mut self, flex_grow: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexGrow(self.inner_node, flex_grow);
		}
	}

	pub fn set_flex_shrink(&mut self, flex_shrink: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexShrink(self.inner_node, flex_shrink);
		}
	}

	pub fn set_flex_basis(&mut self, flex_basis: StyleUnit) {
		unsafe {
			match flex_basis {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetFlexBasis(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetFlexBasis(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetFlexBasisPercent(self.inner_node, val.into_inner())
				}
				StyleUnit::Auto => internal::YGNodeStyleSetFlexBasisAuto(self.inner_node),
			}
		}
	}

	pub fn set_edge_position(&mut self, edge: Edge, position: f32) {
		unsafe {
			internal::YGNodeStyleSetPosition(
				self.inner_node,
				Edge::from(edge),
				position,
			);
		}
	}

	pub fn set_margin(&mut self, edge: Edge, margin: StyleUnit) {
		unsafe {
			match margin {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetMargin(
					self.inner_node,
					Edge::from(edge),
					UNDEFINED,
				),
				StyleUnit::Point(val) => internal::YGNodeStyleSetMargin(
					self.inner_node,
					Edge::from(edge),
					val.into_inner(),
				),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetMarginPercent(
					self.inner_node,
					Edge::from(edge),
					val.into_inner(),
				),
				StyleUnit::Auto => internal::YGNodeStyleSetMarginAuto(
					self.inner_node,
					Edge::from(edge),
				),
			}
		}
	}

	pub fn set_padding(&mut self, edge: Edge, padding: StyleUnit) {
		unsafe {
			match padding {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetPadding(
					self.inner_node,
					Edge::from(edge),
					UNDEFINED,
				),
				StyleUnit::Point(val) => internal::YGNodeStyleSetPadding(
					self.inner_node,
					Edge::from(edge),
					val.into_inner(),
				),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetPaddingPercent(
					self.inner_node,
					Edge::from(edge),
					val.into_inner(),
				),
				// auto is not a valid value for padding
				StyleUnit::Auto => internal::YGNodeStyleSetPadding(
					self.inner_node,
					Edge::from(edge),
					UNDEFINED,
				),
			}
		}
	}

	pub fn set_border(&mut self, edge: Edge, border: f32) {
		unsafe {
			internal::YGNodeStyleSetBorder(self.inner_node, Edge::from(edge), border);
		}
	}

	pub fn set_width(&mut self, width: StyleUnit) {
		unsafe {
			match width {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetWidth(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetWidth(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetWidthPercent(self.inner_node, val.into_inner())
				}
				StyleUnit::Auto => internal::YGNodeStyleSetWidthAuto(self.inner_node),
			}
		}
	}

	pub fn set_height(&mut self, height: StyleUnit) {
		unsafe {
			match height {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetHeight(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetHeight(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetHeightPercent(self.inner_node, val.into_inner())
				}
				StyleUnit::Auto => internal::YGNodeStyleSetHeightAuto(self.inner_node),
			}
		}
	}

	pub fn set_min_width(&mut self, min_width: StyleUnit) {
		unsafe {
			match min_width {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetMinWidth(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetMinWidth(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetMinWidthPercent(self.inner_node, val.into_inner())
				}
				// auto is not a valid value for min_width
				StyleUnit::Auto => internal::YGNodeStyleSetMinWidth(self.inner_node, UNDEFINED),
			}
		}
	}

	pub fn set_min_height(&mut self, min_height: StyleUnit) {
		unsafe {
			match min_height {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetMinHeight(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetMinHeight(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetMinHeightPercent(self.inner_node, val.into_inner())
				}
				// auto is not a valid value for min_height
				StyleUnit::Auto => internal::YGNodeStyleSetMinHeight(self.inner_node, UNDEFINED),
			}
		}
	}

	pub fn set_max_width(&mut self, max_width: StyleUnit) {
		unsafe {
			match max_width {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetMaxWidth(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetMaxWidth(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetMaxWidthPercent(self.inner_node, val.into_inner())
				}
				// auto is not a valid value for max_width
				StyleUnit::Auto => internal::YGNodeStyleSetMaxWidth(self.inner_node, UNDEFINED),
			}
		}
	}

	pub fn set_max_height(&mut self, max_height: StyleUnit) {
		unsafe {
			match max_height {
				StyleUnit::UndefinedValue => {
					internal::YGNodeStyleSetMaxHeight(self.inner_node, UNDEFINED)
				}
				StyleUnit::Point(val) => {
					internal::YGNodeStyleSetMaxHeight(self.inner_node, val.into_inner())
				}
				StyleUnit::Percent(val) => {
					internal::YGNodeStyleSetMaxHeightPercent(self.inner_node, val.into_inner())
				}
				// auto is not a valid value for max_height
				StyleUnit::Auto => internal::YGNodeStyleSetMaxHeight(self.inner_node, UNDEFINED),
			}
		}
	}

	pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
		unsafe {
			internal::YGNodeStyleSetAspectRatio(self.inner_node, aspect_ratio);
		}
	}

	pub fn calculate_layout(
		&mut self,
		available_width: f32,
		available_height: f32,
		parent_direction: Direction,
	) {
		unsafe {
			internal::YGNodeCalculateLayout(
				self.inner_node,
				available_width,
				available_height,
				ffi_types::direction::Direction::from(parent_direction),
			);
		}
	}

	pub fn get_layout(&self) -> Layout {
		unsafe {
			Layout::new(
				internal::YGNodeLayoutGetLeft(self.inner_node),
				internal::YGNodeLayoutGetRight(self.inner_node),
				internal::YGNodeLayoutGetTop(self.inner_node),
				internal::YGNodeLayoutGetBottom(self.inner_node),
				internal::YGNodeLayoutGetWidth(self.inner_node),
				internal::YGNodeLayoutGetHeight(self.inner_node),
			)
		}
	}

	pub fn get_child_count(&self) -> u32 {
		unsafe { internal::YGNodeGetChildCount(self.inner_node) }
	}

	pub fn get_child(&self, index: u32) -> NodeRef {
		unsafe { internal::YGNodeGetChild(self.inner_node, index) }
	}

	pub fn get_style_direction(&self) -> Direction {
		unsafe { internal::YGNodeStyleGetDirection(self.inner_node).into() }
	}

	pub fn get_flex_direction(&self) -> FlexDirection {
		unsafe { internal::YGNodeStyleGetFlexDirection(self.inner_node).into() }
	}

	pub fn get_justify_content(&self) -> Justify {
		unsafe { internal::YGNodeStyleGetJustifyContent(self.inner_node).into() }
	}

	pub fn get_align_content(&self) -> Align {
		unsafe { internal::YGNodeStyleGetAlignContent(self.inner_node).into() }
	}

	pub fn get_align_items(&self) -> Align {
		unsafe { internal::YGNodeStyleGetAlignItems(self.inner_node).into() }
	}

	pub fn get_align_self(&self) -> Align {
		unsafe { internal::YGNodeStyleGetAlignSelf(self.inner_node).into() }
	}

	pub fn get_position_type(&self) -> PositionType {
		unsafe { internal::YGNodeStyleGetPositionType(self.inner_node).into() }
	}

	pub fn get_flex_wrap(&self) -> Wrap {
		unsafe { internal::YGNodeStyleGetFlexWrap(self.inner_node).into() }
	}

	pub fn get_overflow(&self) -> Overflow {
		unsafe { internal::YGNodeStyleGetOverflow(self.inner_node).into() }
	}

	pub fn get_flex_grow(&self) -> f32 {
		unsafe { internal::YGNodeStyleGetFlexGrow(self.inner_node) }
	}

	pub fn get_flex_shrink(&self) -> f32 {
		unsafe { internal::YGNodeStyleGetFlexShrink(self.inner_node) }
	}

	pub fn get_flex_basis(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetFlexBasis(self.inner_node).into() }
	}

	pub fn get_style_position_left(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPosition(self.inner_node, Edge::from(Edge::Left))
				.into()
		}
	}

	pub fn get_style_position_right(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPosition(self.inner_node, Edge::from(Edge::Right))
				.into()
		}
	}

	pub fn get_style_position_top(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPosition(self.inner_node, Edge::from(Edge::Top))
				.into()
		}
	}

	pub fn get_style_position_bottom(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPosition(self.inner_node, Edge::from(Edge::Bottom))
				.into()
		}
	}

	pub fn get_style_position_start(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPosition(self.inner_node, Edge::from(Edge::Start))
				.into()
		}
	}

	pub fn get_style_position_end(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPosition(self.inner_node, Edge::from(Edge::End))
				.into()
		}
	}

	pub fn get_style_margin_left(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetMargin(self.inner_node, Edge::from(Edge::Left))
				.into()
		}
	}

	pub fn get_style_margin_right(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetMargin(self.inner_node, Edge::from(Edge::Right))
				.into()
		}
	}

	pub fn get_style_margin_top(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetMargin(self.inner_node, Edge::from(Edge::Top))
				.into()
		}
	}

	pub fn get_style_margin_bottom(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetMargin(self.inner_node, Edge::from(Edge::Bottom))
				.into()
		}
	}

	pub fn get_style_margin_start(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetMargin(self.inner_node, Edge::from(Edge::Start))
				.into()
		}
	}

	pub fn get_style_margin_end(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetMargin(self.inner_node, Edge::from(Edge::End))
				.into()
		}
	}

	pub fn get_style_padding_left(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPadding(self.inner_node, Edge::from(Edge::Left))
				.into()
		}
	}

	pub fn get_style_padding_right(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPadding(self.inner_node, Edge::from(Edge::Right))
				.into()
		}
	}

	pub fn get_style_padding_top(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPadding(self.inner_node, Edge::from(Edge::Top))
				.into()
		}
	}

	pub fn get_style_padding_bottom(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPadding(self.inner_node, Edge::from(Edge::Bottom))
				.into()
		}
	}

	pub fn get_style_padding_start(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPadding(self.inner_node, Edge::from(Edge::Start))
				.into()
		}
	}

	pub fn get_style_padding_end(&self) -> StyleUnit {
		unsafe {
			internal::YGNodeStyleGetPadding(self.inner_node, Edge::from(Edge::End))
				.into()
		}
	}

	pub fn get_style_border_left(&self) -> f32 {
		unsafe {
			internal::YGNodeStyleGetBorder(self.inner_node, Edge::from(Edge::Left))
		}
	}

	pub fn get_style_border_right(&self) -> f32 {
		unsafe {
			internal::YGNodeStyleGetBorder(self.inner_node, Edge::from(Edge::Right))
		}
	}

	pub fn get_style_border_top(&self) -> f32 {
		unsafe {
			internal::YGNodeStyleGetBorder(self.inner_node, Edge::from(Edge::Top))
		}
	}

	pub fn get_style_border_bottom(&self) -> f32 {
		unsafe {
			internal::YGNodeStyleGetBorder(self.inner_node, Edge::from(Edge::Bottom))
		}
	}

	pub fn get_style_border_start(&self) -> f32 {
		unsafe {
			internal::YGNodeStyleGetBorder(self.inner_node, Edge::from(Edge::Start))
		}
	}

	pub fn get_style_border_end(&self) -> f32 {
		unsafe {
			internal::YGNodeStyleGetBorder(self.inner_node, Edge::from(Edge::End))
		}
	}

	pub fn get_style_width(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetWidth(self.inner_node).into() }
	}

	pub fn get_style_height(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetHeight(self.inner_node).into() }
	}

	pub fn get_style_min_width(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetMinWidth(self.inner_node).into() }
	}

	pub fn get_style_min_height(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetMinHeight(self.inner_node).into() }
	}

	pub fn get_style_max_width(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetMaxWidth(self.inner_node).into() }
	}

	pub fn get_style_max_height(&self) -> StyleUnit {
		unsafe { internal::YGNodeStyleGetMaxHeight(self.inner_node).into() }
	}

	// Layout Getters
	pub fn get_layout_margin_left(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetMargin(self.inner_node, Edge::from(Edge::Left))
		}
	}

	pub fn get_layout_margin_right(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetMargin(self.inner_node, Edge::from(Edge::Right))
		}
	}

	pub fn get_layout_margin_top(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetMargin(self.inner_node, Edge::from(Edge::Top))
		}
	}

	pub fn get_layout_margin_bottom(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetMargin(self.inner_node, Edge::from(Edge::Bottom))
		}
	}

	pub fn get_layout_margin_start(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetMargin(self.inner_node, Edge::from(Edge::Start))
		}
	}

	pub fn get_layout_margin_end(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetMargin(self.inner_node, Edge::from(Edge::End))
		}
	}

	pub fn get_layout_padding_left(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetPadding(self.inner_node, Edge::from(Edge::Left))
		}
	}

	pub fn get_layout_padding_right(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetPadding(self.inner_node, Edge::from(Edge::Right))
		}
	}

	pub fn get_layout_padding_top(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetPadding(self.inner_node, Edge::from(Edge::Top))
		}
	}

	pub fn get_layout_padding_bottom(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetPadding(self.inner_node, Edge::from(Edge::Bottom))
		}
	}

	pub fn get_layout_padding_start(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetPadding(self.inner_node, Edge::from(Edge::Start))
		}
	}

	pub fn get_layout_padding_end(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetPadding(self.inner_node, Edge::from(Edge::End))
		}
	}

	pub fn get_layout_left(&self) -> f32 {
		unsafe { internal::YGNodeLayoutGetLeft(self.inner_node) }
	}

	pub fn get_layout_right(&self) -> f32 {
		unsafe { internal::YGNodeLayoutGetRight(self.inner_node) }
	}

	pub fn get_layout_top(&self) -> f32 {
		unsafe { internal::YGNodeLayoutGetTop(self.inner_node) }
	}

	pub fn get_layout_bottom(&self) -> f32 {
		unsafe { internal::YGNodeLayoutGetBottom(self.inner_node) }
	}

	pub fn get_layout_border_left(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetBorder(self.inner_node, Edge::from(Edge::Left))
		}
	}

	pub fn get_layout_border_right(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetBorder(self.inner_node, Edge::from(Edge::Right))
		}
	}

	pub fn get_layout_border_top(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetBorder(self.inner_node, Edge::from(Edge::Top))
		}
	}

	pub fn get_layout_border_bottom(&self) -> f32 {
		unsafe {
			internal::YGNodeLayoutGetBorder(self.inner_node, Edge::from(Edge::Bottom))
		}
	}

	pub fn get_layout_width(&self) -> f32 {
		unsafe { internal::YGNodeLayoutGetWidth(self.inner_node) }
	}

	pub fn get_layout_height(&self) -> f32 {
		unsafe { internal::YGNodeLayoutGetHeight(self.inner_node) }
	}

	pub fn get_layout_direction(&self) -> Direction {
		unsafe { internal::YGNodeLayoutGetDirection(self.inner_node).into() }
	}

	pub fn is_dirty(&self) -> bool {
		unsafe { internal::YGNodeIsDirty(self.inner_node) }
	}

	pub fn copy_style(&self, src_node: &Node) {
		unsafe { internal::YGNodeCopyStyle(self.inner_node, src_node.inner_node) }
	}

	pub fn set_display(&mut self, display: Display) {
		unsafe {
			internal::YGNodeStyleSetDisplay(self.inner_node, display.into());
		}
	}

	pub fn set_measure_func(&mut self, func: MeasureFunc) {
		match func {
			Some(f) => unsafe {
				type Callback = unsafe extern "C" fn(
					NodeRef,
					f32,
					MeasureMode,
					f32,
					MeasureMode,
				) -> Size;
				let casted_func: Callback = std::mem::transmute(f as usize);
				internal::YGNodeSetMeasureFunc(self.inner_node, Some(casted_func));
			},
			None => unsafe {
				internal::YGNodeSetMeasureFunc(self.inner_node, None);
			},
		}
	}

	pub fn set_baseline_func(&mut self, func: BaselineFunc) {
		match func {
			Some(f) => unsafe {
				type Callback = unsafe extern "C" fn(NodeRef, f32, f32) -> f32;
				let casted_func: Callback = std::mem::transmute(f as usize);
				internal::YGNodeSetBaselineFunc(self.inner_node, Some(casted_func));
			},
			None => unsafe {
				internal::YGNodeSetBaselineFunc(self.inner_node, None);
			},
		}
	}

	pub fn set_context(&mut self, value: Option<Context>) {
		self.drop_context();

		let raw = value.map_or_else(|| std::ptr::null_mut(), |context| context.into_raw());
		unsafe { internal::YGNodeSetContext(self.inner_node, raw) }
	}

	pub fn get_context(node_ref: &NodeRef) -> Option<&Box<dyn Any>> {
		let raw = unsafe { internal::YGNodeGetContext(*node_ref) };
		Context::get_inner_ref(raw)
	}

	pub fn get_context_mut(node_ref: &NodeRef) -> Option<&mut Box<dyn Any>> {
		let raw = unsafe { internal::YGNodeGetContext(*node_ref) };
		Context::get_inner_mut(raw)
	}

	pub fn get_own_context(&self) -> Option<&Box<dyn Any>> {
		Node::get_context(&self.inner_node)
	}

	pub fn get_own_context_mut(&self) -> Option<&mut Box<dyn Any>> {
		Node::get_context_mut(&self.inner_node)
	}

	pub fn drop_context(&mut self) {
		let prev_raw = unsafe { internal::YGNodeGetContext(self.inner_node) };
		Context::drop_raw(prev_raw);
	}
}

impl Drop for Node {
	fn drop(&mut self) {
		self.set_context(None);

		unsafe {
			// In the current revision (a20bde8444474e7a34352a78073de23c26e08fc5),
			// YGNodeFree does not mark the parent as dirty, but YGNodeRemoveChild does.
			// TODO remove the following lines when upgrading to a more recent revision of yoga.
			let parent = internal::YGNodeGetParent(self.inner_node);
			if parent != 0 as NodeRef {
				internal::YGNodeRemoveChild(
					internal::YGNodeGetParent(self.inner_node),
					self.inner_node,
				);
			}

			internal::YGNodeFree(self.inner_node);
		}
	}
}
