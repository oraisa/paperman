use std::process::{Command, Stdio};
use std::io::Write;

pub fn pick(mut selection: json::JsonValue) -> json::JsonValue {
    let keys = selection.entries().map(|(key, _)| key.to_string()).collect::<Vec<_>>();

    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-format")
        .arg("i")
        .arg("-multi-select")
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().expect("Failed to execute rofi");
    let stdin = child.stdin.as_mut().expect("Failed to get rofi stdin");
    for key in &keys {
        selection[key]["title"].write(stdin).expect("Failed to write rofi argument");
        stdin.write(b"\n").expect("Failed to write rofi argument");
    }
    stdin.flush().expect("Failed to flush rofi arguments");

    let output = child.wait_with_output().expect("Failed to wait on rofi");
    let out_str = String::from_utf8(output.stdout).unwrap();

    let selected_indices = out_str.trim().split("\n");
    let selected_keys = selected_indices
        .map(|ind| keys[usize::from_str_radix(ind, 10).expect("Rofi returned invalid index")].clone())
        .collect::<Vec<_>>();
    let to_remove = keys.iter().filter(|key| !selected_keys.contains(key));
    for key in to_remove {
        selection.remove(key);
    }
    return selection;
}
