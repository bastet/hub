FROM ubuntu

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git build-essential

RUN curl -s https://static.rust-lang.org/rustup.sh > /tmp/install-rust.sh
RUN chmod +x /tmp/install-rust.sh
RUN sh /tmp/install-rust.sh --yes

ENV bastet_location="/var/bastet"

RUN mkdir "${bastet_location}"
RUN git clone https://github.com/bastet/bastet-rust.git "${bastet_location}"
RUN cd "${bastet_location}" && cargo build

CMD rustc --version && cargo --version && cd "${bastet_location}" && cargo run
