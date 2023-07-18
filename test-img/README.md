# 测试环境说明
Image来自yjy、cxy编译的linux内核，内核版本为5.4.0。
配套的jailhouse.ko，config文件已经打过RVM1.5的patch，
rootfs来自buildroot
## 启动流程
qemu启动命令为

```sh
qemu-system-aarch64 \
-drive file=./rootfs.qcow2,discard=unmap,if=none,id=disk,format=qcow2 \
-device virtio-blk-device,drive=disk \
-m 1G -serial mon:stdio  \
-kernel Image \
-append "root=/dev/vda mem=768M"  \
-cpu cortex-a57 -smp 4 -nographic -machine virt,gic-version=3,virtualization=on \
-device virtio-serial-device -device virtconsole,chardev=con \
-chardev vc,id=con  \
-net nic \
-net user,hostfwd=tcp::2333-:22
```

qemu启动后，将guest内的文件传入guest linux中
```sh
scp -P 2333 -r test-img/guest/* root@localhost:~/
```
将hypervisor的镜像传入guest
```sh
make scp
```
## guest linux编译说明
### kernel编译
todo
参考

### rootfs编译
todo
使用buildroot
dhcpd
dhcp
网络
dropbear
buildroot里设置rootfs为1G，再把它转为qcow2
qemu-img convert -O qcow2 rootfs.ext4 rootfs.qcow2
配置
