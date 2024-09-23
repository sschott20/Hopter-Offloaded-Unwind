../opt/llvm/bin/llvm-objdump --disassemble --demangle target/thumbv7em-none-eabihf/debug/deps/hopter-765de21662b592af.o > hopter.s 

../opt/llvm/bin/ld.lld --verbose link.ld.in target/thumbv7em-none-eabihf/release/examples/mailbox_uart-f0e9177fdc1cd6f6.o  target/thumbv7e
m-none-eabihf/release/libhopter.rlib 

../opt/llvm/bin/ld.lld --verbose new_link.ld target/thumbv7em-none-eabihf/release/examples/hello_world-734508d427c6609f.o
../opt/llvm/bin/ld.lld --verbose new_link.ld target/thumbv7em-none-eabihf/release/examples/mailbox_uart-f0e9177fdc1cd6f6.o


cargo rustc --release --example hello_world -- --emit=asm
/home/alex/opt/llvm/bin/clang -target thumbv7em-none-eabihf -c target/thumbv7em-none-eabihf/release/examples/hello_world-71420276ad2918ca.s -o asm.o
ld.lld  asm.o  --as-needed -L /home/alex/hopter/target/thumbv7em-none-eabihf/release/deps -L /home/alex/hopter/target/release/deps -L /home/alex/hopter/target/thumbv7em-none-eabihf/release/build/hopter-de8d8993e4e8288a/out -L /home/alex/hopter/target/thumbv7em-none-eabihf/release/build/cortex-m-1d21a837d90dea5e/out -L /home/alex/opt/rust/lib/rustlib/thumbv7em-none-eabihf/lib  /home/alex/hopter/target/thumbv7em-none-eabihf/release/deps/libcompiler_builtins-26e22b02094a596d.rlib -Bdynamic --eh-frame-hdr -z noexecstack -L /home/alex/opt/rust/lib/rustlib/thumbv7em-none-eabihf/lib  --gc-sections -O1 --nmagic -Tlink.ld -o a.out
arm-none-eabi-objcopy  -O binary --pad-to 0 --remove-section=.bss a.out a.bin
qemu-system-arm -machine netduinoplus2 -nographic -semihosting-config enable=on,target=native -kernel a.out


