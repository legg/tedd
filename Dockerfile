FROM rust:1.50

ARG FLAGS
ENV FLAGS=${FLAGS}

WORKDIR /usr/src/untitled
COPY . .

RUN echo "cargo build --release ${FLAGS}" > build.sh && chmod +x build.sh && cat build.sh && ./build.sh && rm build.sh
RUN cargo install --path .
RUN ls -al

CMD ["untitled"]
