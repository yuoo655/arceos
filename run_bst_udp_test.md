编译命令
make A=apps/net/bst_udp_test ARCH=aarch64 SMP=1 PLATFORM=bsta1000b-fada-aarch64 LOG=info FS=y APP_FEATURES=use-ramdisk NET=y fada

cp arceos-fada.itb ~/deploy/

部署命令
adb shell mkdir ~/tmp
adb shell mount /dev/mmcblk0p1 ~/tmp/
adb shell rm -rf ~/tmp/arceos-fada.itb
adb push ./arceos-fada.itb /home/root/
adb shell cp /home/root/arceos-fada.itb ~/tmp/
adb shell sync
adb shell reboot

在uboot阶段暂停 运行arceos
load mmc 0:1 0x90000000 arceos-fada.itb; bootm 0x90000000

arceos起来之后便会一直发送udp包