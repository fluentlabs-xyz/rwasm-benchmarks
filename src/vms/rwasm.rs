use super::{BenchRuntime, BenchVm};
use rwasm::{
    split_i64_to_i32,
    CompilationConfig,
    ExecutionEngine,
    ExecutorConfig,
    RwasmModule,
    Store,
};
use wasmi_new::ModuleImportsIter;

pub struct Rwasm;

impl BenchVm for Rwasm {
    fn name(&self) -> &'static str {
        "rwasm"
    }

    fn compile(&self, wasm: &[u8], _imports: ModuleImportsIter) {
        let config = CompilationConfig::default();
        let _ = RwasmModule::compile(config, wasm).unwrap();
    }

    fn load(&self, wasm: &[u8]) -> Box<dyn BenchRuntime> {
        let config = CompilationConfig::default().with_entrypoint_name("run".into());
        let (module, _) = RwasmModule::compile(config, wasm).unwrap();

        let config = ExecutorConfig::default();
        let store = Store::new(config, ());

        Box::new(RwasmRuntime {
            module,
            store,
            engine: ExecutionEngine::new(),
        })
    }

    fn coremark(&self, wasm: &[u8]) -> f32 {
        todo!()
    }
}

struct RwasmRuntime {
    module: RwasmModule,
    store: Store<()>,
    engine: ExecutionEngine,
}

impl BenchRuntime for RwasmRuntime {
    fn call(&mut self, input: i64) {
        let (input_lo, input_hi) = split_i64_to_i32(input);
        self.engine.value_stack().push(input_lo.into());
        self.engine.value_stack().push(input_hi.into());
        self.engine.execute(&mut self.store, &self.module).unwrap();
    }
}
