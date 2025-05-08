use filter_context::FilterContext;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

mod filter_context;
mod llm_routing;
mod llm_routing_consts;
mod metrics;
mod stream_context;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(FilterContext::new())
    });
}}
