FROM rust



RUN \
      --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
      --mount=target=/var/cache/apt,type=cache,sharing=locked \
      echo 'START apt-get stuff' \
      && apt-get -y update \
      && apt-get install -y \
          'python3' \
          'zip' \
      && echo 'DONE apt-get stuff' ;
