# Debugging

Run `cargo build` then load the code into simavr with 16MHz clock.
```
$ simavr --mcu atmega2560 -f 16000000 -g target/avr-unknown-gnu-atmega2560/debug/aqmon.elf
```

Attach gdb
```
$ avr-gdb
(gdb) file target/avr-unknown-gnu-atmega2560/debug/aqmon.elf
(gdb) target remote :1234
(gdb) layout asm
(gdb) watch *(char*) 0xC6  # watch a register
```

Now set breakpoints and step through code as normal. Use `(gdb) x/xb <ADDR>` to
read the contents of a register at ADDR.

Weird note about the addresses from the author
https://stackoverflow.com/questions/46122094
