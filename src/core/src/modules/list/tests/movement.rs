use ::input::{KeyModifiers, MouseEvent, MouseEventKind};
use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn move_down_1() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3"
			);
		},
	);
}

#[test]
fn move_down_view_end() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown); 2],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}"
			);
		},
	);
}

#[test]
fn move_down_past_end() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown); 3],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}"
			);
		},
	);
}

#[test]
fn move_down_scroll_bottom_move_up_one() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorUp),
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
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3"
			);
		},
	);
}

#[test]
fn move_down_scroll_bottom_move_up_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3"
			);
		},
	);
}

#[test]
fn move_up_attempt_above_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c4"
			);
		},
	);
}

#[test]
fn move_down_attempt_below_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorDown); 4],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn move_page_up_from_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorPageUp)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c4"
			);
		},
	);
}

#[test]
fn move_page_up_from_one_page_down() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c4"
			);
		},
	);
}

#[test]
fn move_page_up_from_one_page_down_minus_1() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c4"
			);
		},
	);
}

#[test]
fn move_page_up_from_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c4"
			);
		},
	);
}

#[test]
fn move_home() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorHome),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c4"
			);
		},
	);
}

#[test]
fn move_end() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorEnd)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn move_page_down_past_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorPageDown); 3],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn move_page_down_one_from_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageDown),
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
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn move_page_down_one_page_from_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn mouse_scroll() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollUp,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
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
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c3"
			);
		},
	);
}

#[test]
fn scroll_right() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::MoveCursorRight)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			test_context.view_context.assert_render_action(&["ScrollRight"]);
		},
	);
}

#[test]
fn scroll_left() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::MoveCursorLeft)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			test_context.view_context.assert_render_action(&["ScrollLeft"]);
		},
	);
}
