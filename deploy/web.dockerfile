FROM node:18-bullseye-slim as base

ENV YARN_CACHE_FOLDER=/cache/yarn

WORKDIR /app

# install dependencies
COPY web/package.json web/yarn.lock ./
RUN --mount=type=cache,target=/cache/yarn \
  yarn install

# copy app source
COPY web/ ./

FROM base as build

RUN ["yarn", "build"]


FROM nginx as web
COPY --from=build /app/dist/ /usr/share/nginx/html/


FROM base as dev

ENV VITE_HOST=0.0.0.0
ENV VITE_PORT=5173
CMD ["yarn", "dev"]
