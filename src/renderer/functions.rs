#[allow(dead_code)]
pub trait BlairStandardFunctions { 
    fn get_url(&self, val: &str) -> String;
    fn get_version(&self) -> String;
    fn get_compile_time(&self) -> String;
}

#[macro_export]
macro_rules! load_BlairStandardFunctions {
    ($($t:ty),+) => {
        $(impl super::functions::BlairStandardFunctions for $t {
                fn get_url(&self, val: &str) -> String {
                    let base_url = config::Configuration::base_url();
                    return format!("{base_url}{val}");
                }

                fn get_version(&self) -> String {
                    return String::from(env!("CARGO_PKG_VERSION"));
                }

                fn get_compile_time(&self) -> String {
                    return String::from("[unimplemented]")
                }
        })+
    }
}

pub(crate) use load_BlairStandardFunctions;