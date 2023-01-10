.text
.section .entry
.global _start
_start:
	# Start stack at 0x80000000 + 1MB
	lui  sp, 0x801
	slli sp, sp, 0x8

	tail main
