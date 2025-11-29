FROM ubuntu:latest
RUN apt update && \
  apt install -y git build-essential nasm libass-dev libfdk-aac-dev libmp3lame-dev libopus-dev libvorbis-dev && \
  git clone --single-branch --branch release/8.0 --depth 1 https://github.com/ffmpeg/ffmpeg && \
  cd ffmpeg && \
  git fetch origin release/8.0 --depth 1 && \
  git checkout FETCH_HEAD && \
  ./configure \
  --enable-libass \
  --enable-libfdk-aac \
  --enable-libfreetype \
  --enable-libmp3lame \
  --enable-libopus \
  --enable-libvorbis \
  --disable-decoder=exr,phm \
  --disable-programs \
  --disable-gpl && \
  make -j && \
  make install
