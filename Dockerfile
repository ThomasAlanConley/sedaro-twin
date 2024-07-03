FROM rust:latest as builder
COPY ./ ./

# The following block is needed (by Tom) to build on the secure INL computer
# This should not be a problem for other computers.  If it is, try commenting
# out this block...
#
RUN wget -q -P /usr/local/share/ca-certificates/ http://certstore.inl.gov/pki/CAINLROOT_B64.crt
RUN /usr/sbin/update-ca-certificates
ENV NODE_EXTRA_CA_CERTS=/etc/ssl/certs/ca-certificates.crt
ENV REQUESTS_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt
ENV CURL_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs/

# build the Sedaro Digital Twin (sedaro-dt)
#
RUN cargo build --release

# get argument, set environment to specify the kind of twin to run
#
ARG SEDARO_TWIN_MODE
ENV SEDARO_TWIN_MODE ${SEDARO_TWIN_MODE}

# run it
#
CMD ./target/release/sedaro-dt -m ${SEDARO_TWIN_MODE}
