use serde_json::Value;

#[derive(Clone, Default, Debug)]
pub struct Settings {
    pub zephyr_extensions: bool,
}

impl Settings {
    pub fn from_json(options: Value) -> Self {
        let mut ops: Settings = Default::default();

        if let Some(extensions) = options.get("zephyr_extensions").and_then(Value::as_bool) {
            ops.zephyr_extensions = extensions;
        }

        ops
    }
}
