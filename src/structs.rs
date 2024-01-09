//derive binary serialization
#[derive(Serialize, Deserialize, Debug)]
struct Clip {
    name: String,
    value: ClipboardItem,
}
