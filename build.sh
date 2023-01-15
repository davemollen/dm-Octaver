DOCKER_BUILDKIT=1 docker build --no-cache --file docker/modduo/Dockerfile --output out/modduo . && \
DOCKER_BUILDKIT=1 docker build --no-cache --file docker/moddwarf/Dockerfile --output out/moddwarf .