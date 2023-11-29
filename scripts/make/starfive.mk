starfive: build
	gzip -9 -cvf $(OUT_BIN) > arceos-starfive.bin.gz
	mkimage -f tools/starfive/starfive_fdt.its arceos.itb
	cp ./arceos.itb ~/tftproot/
	@echo 'Built the FIT-uImage arceos.itb'