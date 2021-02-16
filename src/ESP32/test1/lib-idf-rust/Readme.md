# Readme

This rust library represents the bindings to the esp-idf

```
cargo install bindgen
```

set LIBCLANG_PATH env variable

bindgen --use-core --no-layout-tests --output src/bindings.rs src/bindings.h -- $CLANG_FLAGS


CLANG_FLAGS is set via a for loop involving COMPS, SYSROOT


--sysroot=$SYSROOT -I$(pwd)/build/include -D__bindgen -target xtensa -x c



for INC in `ls -d $COMPS/**/*/include`; do
	#echo $INC
	CLANG_FLAGS+=" -I$INC"
done
for INC in `ls -d $COMPS/*/include`; do
	#echo $INC
	CLANG_FLAGS+=" -I$INC"
done
