.PHONY: dev

dev:
	podman build -t ebpf-firewall-dev .
	podman run -it --rm \
		--pull=never \
		--cap-add=CAP_BPF \
		--cap-add=CAP_NET_ADMIN \
		--device /dev/net/tun \
		-v $$(pwd):/workspace:Z \
		localhost/ebpf-firewall-dev:latest bash
