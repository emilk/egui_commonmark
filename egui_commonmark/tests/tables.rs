//! Snapshot tests for table rendering.
//!
//! Update the snapshots with `UPDATE_SNAPSHOTS=1 cargo test -p egui_commonmark`.

use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_kittest::Harness;

const SIMPLE: &str = "\
| Name  | Role     | Team    |
|-------|----------|---------|
| Ada   | Engineer | Compute |
| Grace | Admiral  | Navy    |
| Linus | Kernel   | Linux   |
";

const WIDE: &str = "\
Text before the table wraps at the panel width, and so does the text after it.

| Key | Value |
|-----|-------|
| short | This is a very long cell with lots of words in it, long enough that it would make the table far wider than the panel unless the text inside the cell wraps onto several lines like a paragraph would. |
| also short | tiny |

Text after the table also wraps at the panel width, even though the table above it has a very long cell in it.
";

const ALIGNMENT: &str = "\
| Left | Center | Right |
|:-----|:------:|------:|
| a    | b      | c     |
| left aligned | centered | right aligned |
| 1    | 22     | 333   |
";

const INLINE_FORMATTING: &str = "\
| Style | Example |
|-------|---------|
| Code | `let x = 1;` |
| Bold | **strong** and _italic_ |
| Link | [egui](https://github.com/emilk/egui) |
| Mixed | `code` then **bold** then [link](https://example.com) |
";

const TOO_NARROW: &str = "\
| One | Two | Three | Four | Five | Six | Seven | Eight |
|-----|-----|-------|------|------|-----|-------|-------|
| first column | second column | third column | fourth column | fifth column | sixth column | seventh column | eighth column |

Text after the table.
";

fn snapshot(name: &str, markdown: &str, width: f32) {
    let mut cache = CommonMarkCache::default();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(width, 300.0))
        .build_ui(|ui| {
            CommonMarkViewer::new().show(ui, &mut cache, markdown);
        });
    harness.run();
    harness.snapshot(name);
}

#[test]
fn table_simple() {
    snapshot("table_simple", SIMPLE, 400.0);
}

#[test]
fn table_wide_wraps() {
    snapshot("table_wide_wraps", WIDE, 400.0);
}

#[test]
fn table_alignment() {
    snapshot("table_alignment", ALIGNMENT, 400.0);
}

#[test]
fn table_inline_formatting() {
    snapshot("table_inline_formatting", INLINE_FORMATTING, 400.0);
}

#[test]
fn table_too_narrow() {
    snapshot("table_too_narrow", TOO_NARROW, 300.0);
}

/// The `commonmark!` macro must render tables the same way as the viewer.
#[cfg(feature = "macros")]
#[test]
fn table_macro() {
    let mut cache = CommonMarkCache::default();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 300.0))
        .build_ui(|ui| {
            egui_commonmark::commonmark!(
                ui,
                &mut cache,
                "\
| Left | Center | Right |
|:-----|:------:|------:|
| a    | b      | c     |
| left aligned | `centered` | **right** aligned |
"
            );
        });
    harness.run();
    harness.snapshot("table_macro");
}
