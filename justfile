default: dev

docker-build:
    docker build --tag=mia-info-poc-app --progress=plain . 2>&1 | tee docker-build.log

dev:
    APP_ENVIRONMENT=local \
    RUST_LIB_BACKTRACE=1 \
    cargo run

get-version bg='ff0000' fg='000000':
    #!/usr/bin/env bash
    set -euo pipefail
    url='http://localhost:8080/deployment/moa-dev/myqueue-contacts/version/badge?bg={{bg}}&fg={{fg}}'
    img_b64="$(curl -ksSL "$url" | base64)"
    printf '\e_Ga=T,f=100,t=d,s=0,v=0,S=0,O=0,I=0,m=0;%s\e\\\n' "$img_b64"

get-containers bg='ff40ff' fg='000000':
    #!/usr/bin/env bash
    set -euo pipefail
    url='http://localhost:8080/deployment/moa-dev/myqueue-contacts/containers/badge?bg={{bg}}&fg={{fg}}'
    img_b64="$(curl -ksSL "$url" | base64)"
    printf '\e_Ga=T,f=100,t=d,s=0,v=0,S=0,O=0,I=0,m=0;%s\e\\\n' "$img_b64"
