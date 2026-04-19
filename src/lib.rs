use marker::fun;
use plugin::call;

#[derive(Default)]
pub struct PluginDeveloper;

#[call]
impl PluginDeveloper {
    #[fun]
    pub async fn call_hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}
