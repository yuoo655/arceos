# make A=apps/net/bst_nic_test ARCH=x86_64 NET=y LOG=info run


make A=apps/net/bst_udp_test ARCH=aarch64 SMP=1 PLATFORM=bsta1000b-fada-aarch64 LOG=info FS=y APP_FEATURES=use-ramdisk NET=y fada

cp arceos-fada.itb ~/deploy/



make A=apps/net/bst_nic_test ARCH=aarch64 SMP=1 PLATFORM=aarch64-bsta1000b LOG=info FS=y APP_FEATURES=use-ramdisk NET=y fada
