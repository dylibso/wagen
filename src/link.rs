use crate::*;

pub struct ExtismFunctions {
    pub input_load_u8: FunctionIndex,
    pub input_load_u64: FunctionIndex,
    pub input_length: FunctionIndex,
    pub length: FunctionIndex,
    pub length_unsafe: FunctionIndex,
    pub alloc: FunctionIndex,
    pub free: FunctionIndex,
    pub output_set: FunctionIndex,
    pub error_set: FunctionIndex,
    pub config_get: FunctionIndex,
    pub var_get: FunctionIndex,
    pub var_set: FunctionIndex,
    pub store_u8: FunctionIndex,
    pub load_u8: FunctionIndex,
    pub store_u64: FunctionIndex,
    pub load_u64: FunctionIndex,
    pub http_request: FunctionIndex,
    pub http_status_code: FunctionIndex,
    pub log_info: FunctionIndex,
    pub log_debug: FunctionIndex,
    pub log_warn: FunctionIndex,
    pub log_error: FunctionIndex,
}

impl<'a> Module<'a> {
    pub fn link_extism(&mut self) -> ExtismFunctions {
        let input_load_u8 = self.import(
            "extism:host/env",
            "input_load_u8",
            None,
            [ValType::I64],
            [ValType::I32],
        );
        let input_load_u64 = self.import(
            "extism:host/env",
            "input_load_u64",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let input_length = self.import("extism:host/env", "input_length", None, [], [ValType::I64]);
        let length = self.import(
            "extism:host/env",
            "length",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let length_unsafe = self.import(
            "extism:host/env",
            "length",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let alloc = self.import(
            "extism:host/env",
            "alloc",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let free = self.import("extism:host/env", "free", None, [ValType::I64], []);
        let output_set = self.import(
            "extism:host/env",
            "output_set",
            None,
            [ValType::I64, ValType::I64],
            [],
        );
        let error_set = self.import("extism:host/env", "error_set", None, [ValType::I64], []);
        let config_get = self.import(
            "extism:host/env",
            "config_get",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let var_get = self.import(
            "extism:host/env",
            "var_get",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let var_set = self.import(
            "extism:host/env",
            "var_set",
            None,
            [ValType::I64, ValType::I64],
            [],
        );
        let store_u8 = self.import(
            "extism:host/env",
            "store_u8",
            None,
            [ValType::I64, ValType::I32],
            [],
        );
        let load_u8 = self.import(
            "extism:host/env",
            "load_u8",
            None,
            [ValType::I64],
            [ValType::I32],
        );
        let store_u64 = self.import(
            "extism:host/env",
            "store_u64",
            None,
            [ValType::I64, ValType::I64],
            [],
        );
        let load_u64 = self.import(
            "extism:host/env",
            "load_u64",
            None,
            [ValType::I64],
            [ValType::I64],
        );
        let http_request = self.import(
            "extism:host/env",
            "http_request",
            None,
            [ValType::I64, ValType::I64],
            [ValType::I64],
        );
        let http_status_code = self.import(
            "extism:host/env",
            "http_status_code",
            None,
            [],
            [ValType::I32],
        );

        let log_info = self.import("extism:host/env", "log_info", None, [ValType::I64], []);
        let log_debug = self.import("extism:host/env", "log_info", None, [ValType::I64], []);
        let log_warn = self.import("extism:host/env", "log_info", None, [ValType::I64], []);
        let log_error = self.import("extism:host/env", "log_info", None, [ValType::I64], []);

        ExtismFunctions {
            input_load_u8,
            input_load_u64,
            input_length,
            length,
            length_unsafe,
            alloc,
            free,
            output_set,
            error_set,
            config_get,
            var_get,
            var_set,
            store_u8,
            load_u8,
            store_u64,
            load_u64,
            http_request,
            http_status_code,
            log_info,
            log_debug,
            log_warn,
            log_error,
        }
    }
}
