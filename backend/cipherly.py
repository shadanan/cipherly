from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

app = FastAPI()


@app.get("/api/hello")
def read_root():
    return {"Hello": "World"}


app.mount("/", StaticFiles(directory="frontend", html=True), name="static")
