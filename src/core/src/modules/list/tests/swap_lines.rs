use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn normal_mode_change_swap_down() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn normal_mode_change_swap_up() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}
