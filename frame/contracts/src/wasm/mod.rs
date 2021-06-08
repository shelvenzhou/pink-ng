
/*
impl<T: Config> Executable<T> for PrefabWasmModule<T>
where
	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>
{
	fn from_storage(
		code_hash: CodeHash<T>,
		schedule: &Schedule<T>,
		gas_meter: &mut GasMeter<T>,
	) -> Result<Self, DispatchError> {
		code_cache::load(code_hash, Some((schedule, gas_meter)))
	}

	fn from_storage_noinstr(code_hash: CodeHash<T>) -> Result<Self, DispatchError> {
		code_cache::load(code_hash, None)
	}

	fn drop_from_storage(self) {
		code_cache::store_decremented(self);
	}

	fn add_user(code_hash: CodeHash<T>) -> Result<u32, DispatchError> {
		code_cache::increment_refcount::<T>(code_hash)
	}

	fn remove_user(code_hash: CodeHash<T>) -> u32 {
		code_cache::decrement_refcount::<T>(code_hash)
	}

	fn execute<E: Ext<T = T>>(
		self,
		ext: &mut E,
		function: &ExportedFunction,
		input_data: Vec<u8>,
	) -> ExecResult {
		let memory =
			sp_sandbox::Memory::new(self.initial, Some(self.maximum))
				.unwrap_or_else(|_| {
				// unlike `.expect`, explicit panic preserves the source location.
				// Needed as we can't use `RUST_BACKTRACE` in here.
					panic!(
						"exec.prefab_module.initial can't be greater than exec.prefab_module.maximum;
						thus Memory::new must not fail;
						qed"
					)
				});

		let mut imports = sp_sandbox::EnvironmentDefinitionBuilder::new();
		imports.add_memory(self::prepare::IMPORT_MODULE_MEMORY, "memory", memory.clone());
		runtime::Env::impls(&mut |module, name, func_ptr| {
			imports.add_host_func(module, name, func_ptr);
		});

		let mut runtime = Runtime::new(
			ext,
			input_data,
			memory,
		);

		// We store before executing so that the code hash is available in the constructor.
		let code = self.code.clone();
		if let &ExportedFunction::Constructor = function {
			code_cache::store(self)
		}

		// Instantiate the instance from the instrumented module code and invoke the contract
		// entrypoint.
		let result = sp_sandbox::Instance::new(&code, &imports, &mut runtime)
			.and_then(|mut instance| instance.invoke(function.identifier(), &[], &mut runtime));

		runtime.to_execution_result(result)
	}

	fn code_hash(&self) -> &CodeHash<T> {
		&self.code_hash
	}

	fn code_len(&self) -> u32 {
		self.code.len() as u32
	}

	fn aggregate_code_len(&self) -> u32 {
		self.original_code_len.saturating_add(self.code_len())
	}

	fn refcount(&self) -> u32 {
		self.refcount as u32
	}
}
*/
