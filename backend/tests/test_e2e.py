from fastapi.testclient import TestClient

from cipherly import app

client = TestClient(app)


def test_read_root():
    response = client.get("/api/hello")
    assert response.status_code == 200
    assert response.json() == {"Hello": "World"}
