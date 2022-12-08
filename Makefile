run:
	docker run --rm -it -e PERF=perf_5.10 --cap-add=SYS_PTRACE --cap-add=SYS_ADMIN --security-opt seccomp=unconfined -v $(CURDIR):/home/map rustaoc /bin/bash

build:
	docker build -t rustaoc .

