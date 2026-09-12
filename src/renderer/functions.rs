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
            /* =============
               = START! 🔊 =
               ============= */

            fn get_url(&self, val: &str) -> String {
                let base_url = &BLAIR.current_base_url;
                return format!("{base_url}{val}");
            }

            fn get_version(&self) -> String {
                return String::from(env!("CARGO_PKG_VERSION"));
            }

            fn get_compile_time(&self) -> String {
                return BLAIR.compile_time.format("%Y/%m/%d %H:%M").to_string()
            }

            fn is_debug(&self) -> bool {
                let mode = &BLAIR.running_mode;
                match mode {
                    crate::config::RunningMode::Debug => return true,
                    crate::config::RunningMode::Production => return false,
                }
            }

            /* ==============
               = FINISH! 🏁 =
               ============== */
        })+
    }
}

pub(crate) use load_BlairStandardFunctions;
