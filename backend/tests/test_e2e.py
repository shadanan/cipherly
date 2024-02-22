from fastapi.testclient import TestClient

from cipherly import app

client = TestClient(app)


def test_read_root():
    response = client.get("/api/hello")
    assert response.status_code == 200
    assert response.json() == {"Hello": "World"}


def test_encrypt_decrypt():
    response = client.post(
        "/api/encrypt",
        json={
            "dek": "ZGVr",
            "iv": "aXY=",
            "authorized_users": ["user1", "user2"],
        },
    )
    assert response.status_code == 200
    envelope_encrypted = response.json()
    response = client.post("/api/decrypt", json=envelope_encrypted)
    assert response.status_code == 200
    dek = response.json()
    assert dek == {
        "dek": "ZGVr",
        "iv": "aXY=",
    }
