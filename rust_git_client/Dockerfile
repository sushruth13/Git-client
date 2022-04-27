FROM ubuntu:21.10

WORKDIR /home

RUN apt-get update
RUN apt-get install libssl-dev
RUN apt install pkg-config -y
RUN aot-get install git -y
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get -y install tzdata
RUN apt-get install -y curl gcc
RUN mkdir ~/.ssh/
RUN chmod 700 ~/.ssh
RUN apt-get install git -y
RUN apt-get install vim -y
RUN apt install openssh-client -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://getmic.ro | sh -s -- -y && mv micro /usr/bin/
WORKDIR gitclient/
COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build

CMD ["/bin/bash"]
