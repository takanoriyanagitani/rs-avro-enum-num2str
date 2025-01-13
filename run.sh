#!/bin/sh

export ENV_ENUM_COLUMN=status

run_native(){
	export ENV_ENUM_INDEX=0
	cat sample.d/input.avsc | ./rs-avro-enum-num2str
}

run_wasmer(){
	cat sample.d/input.avsc |
		wasmer \
			run \
			--env ENV_ENUM_INDEX=1 \
			--env ENV_ENUM_COLUMN=status \
			./rs-avro-enum-num2str.wasm
}

run_wasmtime(){
	cat sample.d/input.avsc |
		wasmtime \
			run \
			--env ENV_ENUM_INDEX=2 \
			--env ENV_ENUM_COLUMN=status \
			./rs-avro-enum-num2str.wasm
}

run_wazero(){
	cat sample.d/input.avsc |
		wazero \
			run \
			-env ENV_ENUM_INDEX=0 \
			-env ENV_ENUM_COLUMN=status \
			./rs-avro-enum-num2str.wasm
}

run_native
run_wasmer
run_wasmtime
run_wazero
