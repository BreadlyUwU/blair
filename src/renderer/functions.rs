#[allow(dead_code)]
pub trait BlairStandardFunctions {
    fn get_url(&self, val: &str) -> String;
    fn get_version(&self) -> String;
    fn get_compile_time(&self) -> String;
    fn is_debug(&self) -> bool;
}

#[macro_export]
macro_rules! load_BlairStandardFunctions {
    ($($t:ty),+) => {
        $(impl functions::BlairStandardFunctions for $t {
            // =============
            // = START! 🔊 =
            // =============

            fn get_url(&self, val: &str) -> String {
                let base_url = config::init().current_base_url;
                return format!("{base_url}{val}");
            }

            fn get_version(&self) -> String {
                return String::from(env!("CARGO_PKG_VERSION"));
            }

            fn get_compile_time(&self) -> String {
                return String::from("[unimplemented]")
            }

            fn is_debug(&self) -> bool {
                let mode = config::init().running_mode;
                match mode {
                    crate::config::RunningMode::Debug => return true,
                    crate::config::RunningMode::Production => return false,
                }
            }

            // ==============
            // = FINISH! 🏁 =
            // ==============
        })+
    }
}

pub(crate) use load_BlairStandardFunctions;
