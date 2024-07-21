use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let main = document.create_element("main")?;
    let el = document.create_element("p")?;
    el.set_text_content(Some("blorf"));

    main.append_child(&el)?;
    body.append_child(&main)?;

    Ok(())
}
