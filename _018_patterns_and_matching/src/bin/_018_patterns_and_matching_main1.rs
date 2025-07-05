fn main() {
    assert_eq!(
        "Using your favorite, sutiany, as the background",
        &color(String::from("34"), false, Some("sutiany"))
    );
    assert_eq!(
        "Tuesday is green day!",
        &color(String::from("34"), true, None)
    );
    assert_eq!(
        "Using purple as the background color",
        &color(String::from("34"), false, None)
    );
    assert_eq!(
        "Using orange as the background color",
        &color(String::from("30"), false, None)
    );
    assert_eq!(
        "Using blue as the background color",
        &color(String::from("3c4"), false, None)
    );
}
fn color(age: String, is_tuesday: bool, favorite_color: Option<&str>) -> String {
    // IMPORTANT
    let age: Result<u8, _> = age.parse();

    if let Some(color) = favorite_color {
        return format!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        return format!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            return format!("Using purple as the background color");
        } else {
            return format!("Using orange as the background color");
        }
    } else {
        return format!("Using blue as the background color");
    }
}
