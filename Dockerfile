FROM        ubuntu:xenial


################################################################################
# Basic Development Tools
################################################################################

RUN     apt-get update -qq
RUN     apt-get upgrade -qq

RUN     apt-get install -qq wget
RUN     apt-get install -qq build-essential gcc


################################################################################
# Travis
################################################################################

RUN     apt-get install -qq ruby ruby-dev
RUN     gem install travis


################################################################################
# ctags
################################################################################

RUN     apt-get install -qq ctags


################################################################################
# Rust
################################################################################

RUN    apt-get install -qq curl graphviz

RUN    curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN    echo '\nexport PATH=$PATH:/root/.cargo/bin\n' >> /root/.bashrc

RUN    /root/.cargo/bin/cargo install rustfmt
RUN    /root/.cargo/bin/cargo install cargo-watch
RUN    /root/.cargo/bin/cargo install cargo-outdated
RUN    /root/.cargo/bin/cargo install cargo-graph
RUN    /root/.cargo/bin/cargo install cargo-modules
RUN    /root/.cargo/bin/cargo install cargo-count

RUN    /root/.cargo/bin/rustup install nightly
RUN    /root/.cargo/bin/rustup run nightly cargo install clippy
