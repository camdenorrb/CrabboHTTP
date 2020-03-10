struct HTML {

    // Map of elements  head tag -> child tag
}


enum HTMLElement {

    // value
    // list of child elements

    PARAGRAPH(str)

}

// TODO: Add validity checks

fn loadHTML() -> HTML {

}

// Example input: <p>Test</p>
fn loadElementByText(id: &str) -> HTMLElement {
    match id {
        "p" => PARAGRAPH
    }
}