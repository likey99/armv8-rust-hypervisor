# RVM1.5 arm移植

使用rust写的armv8 hypervisor，Porting from 
https://github.com/rcore-os/RVM1.5

## 环境配置
### 安装rust
首先安装 Rust 版本管理器 rustup 和 Rust 包管理器 cargo，为了在国内加速访问，可以设置使用中科大的镜像服务器。
```sh
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl https://sh.rustup.rs -sSf | sh  
```
最好把 Rust 包管理器 cargo 镜像地址 crates.io 也替换成中国科学技术大学的镜像服务器，来加速三方库的下载。 打开或新建 ~/.cargo/config 文件，并把内容修改为：
```sh
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```
### qemu模拟器安装
```sh
sudo apt-get update
sudo apt-get install qemu
sudo apt-get install qemu-system-arm
```
### 启动qemu
```sh
mkdir qemu-test    # 新建一个文件夹用来测试
cp -r test-img/* qemu-test   #将所需的文件传入测试文件夹
cd qemu-test
cd host
./test.sh    #启动qemu
```
linux默认用户密码为root/root
### 编译sysHyper
在host执行
```sh
make     #编译得到hypervisor镜像rvmarm.bin
make scp   #将得到的rvmarm.bin文件传入qemu上运行的linux
```
### 运行sysHyper
将必要的文件传入guest linux：
```sh
scp -P 2333 -r qemu-test/guest/* root@localhost:~/
```
在guest linux中
```sh
./setup.sh  #设置文件路径
./enable.sh   #运行sysHyper，开启虚拟化
cat /proc/cpuinfo   #查看当前linux cpuinfo
jailhouse cell create configs/qemu-arm64-gic-demo.cell  #新建一个cell，将cpu 3 移出root cell
cat /proc/cpuinfo   #查看当前linux cpuinfo，cpu3被shutdown了
jailhouse disable  # 关闭虚拟化
```
### output
应该可以看到hypervisor运行打印的一些信息


### 调试
参考文档仓库

本项目的相关文档在
https://github.com/syswonder/report