use xcap::Monitor;

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}

fn get_monitors() -> Vec<Monitor> {
    let monitors = Monitor::all().unwrap();

    return monitors;
}