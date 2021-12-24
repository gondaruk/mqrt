FROM rustembedded/cross:x86_64-unknown-linux-gnu

# Make rquickjs-sys happy
# See https://stackoverflow.com/questions/20326604/stdatomic-h-in-gcc-4-8
RUN yum update -y && \
    yum install patch -y && \
    yum install centos-release-scl -y && \
    yum clean all && \
    yum install devtoolset-9-* -y

COPY entrypoint-rhel.sh /usr/bin/entrypoint.sh
RUN chmod +x /usr/bin/entrypoint.sh

ENTRYPOINT ["/usr/bin/entrypoint.sh"]
