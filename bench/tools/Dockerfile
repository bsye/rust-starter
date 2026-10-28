FROM mcr.microsoft.com/playwright:v1.64.1-noble
ARG LIGHTHOUSE_VERSION=13.6.0
ARG PLAYWRIGHT_VERSION=1.64.1
WORKDIR /tools
RUN npm install --global --no-fund --no-audit "lighthouse@${LIGHTHOUSE_VERSION}" \
    && npm init --yes > /dev/null \
    && npm install --no-fund --no-audit "playwright@${PLAYWRIGHT_VERSION}"
