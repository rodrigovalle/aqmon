.PHONY: upload

BIN=aqmon
DEVICE=/dev/ttyUSB0
RELEASE_DIR=target/avr-unknown-gnu-atmega2560/release
ELF=$(RELEASE_DIR)/$(BIN).elf
HEX=$(RELEASE_DIR)/$(BIN).hex
EEP=$(RELEASE_DIR)/$(BIN).eep
SRC=$(wildcard **/*.rs)

build: $(HEX)

upload: $(HEX)
	avrdude -patmega2560 -cwiring -P$(DEVICE) -b115200 -D -Uflash:w:$(HEX):i

# rust builds a .elf file
$(ELF): $(SRC)
	cargo build --release

# convert elf to hex for the programmer, pull .eeprom out because that is
# programmed separately
# ihex is short for "intel hex", which is recognized by avrdude
$(HEX): $(ELF)
	avr-objcopy -O ihex -R .eeprom $(ELF) $(HEX)

# pull .eeprom from the elf and create a separate file representing what should
# be written to the eeprom (this file is currently unused in the build process)
# eeprom can also be programmed by avrdude's -U
$(EEP): $(ELF)
	avr-objcopy -O ihex -j .eeprom --set-section-flags=.eeprom=alloc,load --no-change-warnings --change-section-lma .eeprom=0 $(ELF) $(EEP)
