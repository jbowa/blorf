use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let el = document.create_element("main")?;
    el.set_text_content(Some("blorf"));

    body.append_child(&el)?;

    Ok(())
}
