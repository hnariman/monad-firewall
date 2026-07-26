VM_NAME = mini-vm
BUILD_DIR ?= target/debug

.PHONY: dev

dev:
	podman build -t ebpf-firewall-dev .
	podman run -it --rm \
		--pull=never \
		--cap-add=CAP_BPF \
		--cap-add=CAP_NET_ADMIN \
		--device /dev/kvm:/dev/kvm \
		--device /dev/net/tun \
		-v $$(pwd):/workspace:Z \
		localhost/ebpf-firewall-dev:latest bash

build:
	cargo +nightly build \
		--package monad-firewall-ebpf \
		--target bpfel-unknown-none \
		--release \
		-Z build-std=core
	cargo build --package monad-firewall-core

get-vm-iso:
	 curl -L -o lima/Arch-Linux-x86_64-cloudimg.qcow2 \
		 "https://geo.mirror.pkgbuild.com/images/latest/Arch-Linux-x86_64-cloudimg.qcow2"

vm-start:
	# @if ! limactl list | grep -q "$(VM_NAME)"; then \
		#
	# limactl start template:archlinux --name $(VM_NAME)
	limactl start /etc/lima/mini-vm.yml --name $(VM_NAME) --tty=false
	# else \
	# 	limactl start $(VM_NAME); \
	# fi

vm-deploy:
	@echo "deploying ebpf into vm"
	limactl copy $(BUILD_DIR)/monad-firewall $(VM_NAME):/tmp/fw-daemon
	limactl copy lima/fw.service $(VM_NAME):/tmp/fw.service
	@echo "spin up fw"
	limactl shell $(VM_NAME) sudo mv /tmp/fw-daemon /usr/local/bin/fw-daemon
	limactl shell $(VM_NAME) sudo chmod +x /usr/local/bin/fw-daemon
	limactl shell $(VM_NAME) sudo mv /tmp/fw.service /etc/systemd/system/fw.service
	limactl shell $(VM_NAME) sudo chmod +x /etc/systemd/system/fw.service
	limactl shell $(VM_NAME) sudo systemctl restart fw.service

vm-logs:
	limactl shell $(VM_NAME) sudo journalctl -u fw.service -f -o cat

vm-status:
	limactl shell $(VM_NAME) sudo systemctl status fw.service

vm-stop:
	limactl stop $(VM_NAME)

vm-destroy:
	limactl delete -f $(VM_NAME)

clean:
	rm -rf target

help:
	@echo "available options:"
	@echo "dev"
	@echo "build"
	@echo "vm-start"
	@echo "vm-deploy"
	@echo "vm-logs"
	@echo "vm-status"
	@echo "vm-stop"
	@echo "vm-destroy"
	@echo "clean"
