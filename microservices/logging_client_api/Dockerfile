FROM python:3.14

WORKDIR /code

COPY ./dependencies.txt /code/dependencies.txt

RUN pip install --no-cache-dir --upgrade -r dependencies.txt

COPY ./app /code/app

CMD ["fastapi", "run", "app/main.py", "--port", "5000"]
