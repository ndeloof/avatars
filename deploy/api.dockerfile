FROM --platform=${BUILDPLATFORM} python:3.10-slim-bullseye

ENV PIP_DISABLE_PIP_VERSION_CHECK="1"
RUN apt update && apt install -y git

WORKDIR /app
COPY api/requirements.txt ./
RUN mkdir -p -m 0700 ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts
RUN --mount=type=cache,target=/root/.cache/pip \
    --mount=type=ssh,required=true \
  pip install -r requirements.txt

COPY api/ ./api/

ENV FLASK_APP=./api/app.py
ENV FLASK_ENV=development

CMD [ "flask", "run", "--host=0.0.0.0", "--port=80" ]
