ARG NODE_VERSION=26
FROM node:${NODE_VERSION}-trixie-slim AS base
WORKDIR /app

FROM base AS dev
EXPOSE 3000
CMD ["sh", "-c", "npm install --no-fund --no-audit && npm run dev -- --host 0.0.0.0"]

FROM base AS builder
COPY package.json package-lock.json ./
RUN --mount=type=cache,target=/root/.npm npm ci --no-fund --no-audit --ignore-scripts
COPY . .
RUN NITRO_PRESET=node_cluster npm run build

FROM node:${NODE_VERSION}-trixie-slim AS prod
ENV NODE_ENV=production HOST=0.0.0.0 PORT=8080 NUXT_MIGRATIONS_DIR=/app/migrations
WORKDIR /app
COPY --from=builder /app/.output ./.output
COPY --from=builder /app/server/db/migrations ./migrations
USER node
EXPOSE 8080
HEALTHCHECK --interval=10s --timeout=3s --start-period=10s CMD node -e "fetch('http://127.0.0.1:8080/health').then(r => process.exit(r.ok ? 0 : 1)).catch(() => process.exit(1))"
ENTRYPOINT ["node", ".output/server/index.mjs"]
