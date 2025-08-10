docker-build:
    docker build --tag=mia-info-poc-app --progress=plain . 2>&1 | tee docker-build.log
