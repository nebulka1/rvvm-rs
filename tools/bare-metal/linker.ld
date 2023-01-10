MEMORY {
	rom : ORIGIN = 0x80000000, LENGTH = 512K
}

SECTIONS {
	.text : {
		*(.entry)
		*(.text*)
		*(.rodata*)
		*(.sdata*)
	} > rom
}
