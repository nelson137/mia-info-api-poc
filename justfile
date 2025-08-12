default: dev

docker-build:
    docker build --tag=mia-info-poc-app --progress=plain . 2>&1 | tee docker-build.log

dev:
    APP_ENVIRONMENT=local cargo run

get-version:
    #!/usr/bin/env bash
    set -euo pipefail
    url='http://localhost:8080/deployment/moa-dev/myqueue-contacts/version/badge'
    img_b64="$(curl -ksSL "$url" | base64)"
    printf '\e_Ga=T,f=100,t=d,s=0,v=0,S=0,O=0,I=0,m=0;%s\e\\\n' "$img_b64"

get-containers:
    #!/usr/bin/env bash
    set -euo pipefail
    url='http://localhost:8080/deployment/moa-dev/myqueue-contacts/containers/badge'
    img_b64="$(curl -ksSL "$url" | base64)"
    printf '\e_Ga=T,f=100,t=d,s=0,v=0,S=0,O=0,I=0,m=0;%s\e\\\n' "$img_b64"
