DOCKER_BUILDKIT=1 docker build --file docker/modduo/Dockerfile --output out/modduo . && \
DOCKER_BUILDKIT=1 docker build --file docker/moddwarf/Dockerfile --output out/moddwarf .
docker builder prune --force