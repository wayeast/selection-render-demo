use seed::{prelude::*, *};

use web_sys::KeyboardEvent;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        text: "Click in me and type a letter.".to_string(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    text: String,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    KeyDown(KeyboardEvent),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::KeyDown(ev) => {
            let (selection, node, offset) = anchor_things().unwrap();
            let (_, key, _, _, _) = key_things(&ev);

            // This action triggers a re-render, which places the user's caret at the beginning of
            // the div
            model.text.insert_str(offset as usize, &key);

            // This is one attempt to set the cursor at the right location, but this happens before
            // a re-render, so is useless.  Comment it out or not, it makes no difference.
            let range = document().create_range().unwrap();
            range.set_start(&node, offset + 1).unwrap();
            range.collapse();
            selection.remove_all_ranges().unwrap();
            selection.add_range(&range).unwrap();
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    log!("rendering model with text:", model.text);
    div![
        attrs!{
            At::ContentEditable => true.as_at_value();
        },
        &model.text,
        keyboard_ev(Ev::KeyDown, |e| {
            Msg::KeyDown(e)
        }),
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

// ------ ------
//   Supplements
// ------ ------

fn key_things(ev: &web_sys::KeyboardEvent) -> (String, String, bool, bool, bool) {
    (ev.code(), ev.key(), ev.ctrl_key(), ev.alt_key(), ev.shift_key())
}

fn anchor_things() -> Result<(web_sys::Selection, web_sys::Node, u32), ()> {
    // find caret position
    let selection = window().get_selection().map_err(|_| {
        log!("ERROR: unable to find selection in document!");
    })?.ok_or_else(|| {
        log!("ERROR: no selection found!");
    })?;

    // the anchor node is the inner text of the span element
    let node = selection.anchor_node().ok_or_else(|| {
        log!("ERROR: no anchor node found from selection!");
    })?;

    let offset = selection.anchor_offset();

    Ok((selection, node, offset))
}
