FROM ubuntu:14.04
MAINTAINER Parity Technologies <devops@parity.io>
WORKDIR /build
#ENV for build TAG
ARG BUILD_TAG
ENV BUILD_TAG ${BUILD_TAG:-master}
RUN echo "Build tag:" $BUILD_TAG
# install ssh
RUN apt-get update && \
	apt-get install -y --force-yes --no-install-recommends openssh-client
# add credentials on build
ARG SSH_PRIVATE_KEY
RUN mkdir /root/.ssh/ && \
	echo "${SSH_PRIVATE_KEY}" > /root/.ssh/id_rsa && \
	chmod 700 /root/.ssh/id_rsa && \
	touch /root/.ssh/known_hosts && \
	ssh-keyscan gricad-gitlab.univ-grenoble-alpes.fr >> /root/.ssh/known_hosts
# install tools and dependencies
RUN apt-get install -y --force-yes --no-install-recommends \
        # make
        build-essential \
        # add-apt-repository
        software-properties-common \
        make \
        curl \
        wget \
        git \
        g++ \
        gcc \
        libc6 \
        libc6-dev \
        binutils \
        file \
        openssl \
        libssl-dev \
        libudev-dev \
        pkg-config \
        dpkg-dev \
        libudev-dev &&\
# install rustup
 curl https://sh.rustup.rs -sSf | sh -s -- -y && \
# rustup directory
 PATH=/root/.cargo/bin:$PATH && \
# show backtraces
 RUST_BACKTRACE=1 && \
# build parity
cd /build&&git clone git@gricad-gitlab.univ-grenoble-alpes.fr:mertzweh/parity.git && \
        cd parity && \
	git pull&& \
	git checkout $BUILD_TAG && \
        cargo build --release && \
        #ls /build/parity/target/release/parity && \
        strip /build/parity/target/release/parity && \
 file /build/parity/target/release/parity&&mkdir -p /parity&& cp /build/parity/target/release/parity /parity&&\
#cleanup Docker image
 rm -rf /root/.cargo&&rm -rf /root/.multirust&&rm -rf /root/.rustup&&rm -rf /build&&\
 apt-get purge -y  \
        # make
        build-essential \
        # add-apt-repository
        software-properties-common \
        make \
        curl \
        wget \
        git \
        g++ \
        gcc \
        binutils \
        file \
        pkg-config \
        dpkg-dev &&\
 rm -rf /var/lib/apt/lists/*
# setup ENTRYPOINT
EXPOSE 8080 8545 8180
ENTRYPOINT ["/parity/parity"]
