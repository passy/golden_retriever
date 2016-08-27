all: build

docker: Dockerfile
	docker build --rm=true -t grrr .

shell:
	docker run -it -v "$(CURDIR):/grrr" --workdir=/grrr grrr /bin/bash


.PHONY: docker shell
