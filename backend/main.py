import base64
import os

from fastapi import FastAPI
from google.cloud import kms
from pydantic import BaseModel

KMS_KEY_NAME = os.environ["KMS_KEY_NAME"]


class EnvelopeEncrypted(BaseModel):
    header: str


class Envelope(BaseModel):
    dek: dict
    iv: list[int]
    authorized_users: list[str]


class Dek(BaseModel):
    dek: dict
    iv: list[int]


app = FastAPI()


@app.get("/api/hello")
def read_root():
    return {"Hello": "World"}


@app.post("/api/encrypt")
def encrypt(request: Envelope) -> EnvelopeEncrypted:
    client = kms.KeyManagementServiceClient()
    envelope_serialized = request.model_dump_json().encode()
    kms_response = client.encrypt(
        request={
            "name": KMS_KEY_NAME,
            "plaintext": envelope_serialized,
        }
    )
    envelope_encrypted = base64.urlsafe_b64encode(kms_response.ciphertext).decode()
    return EnvelopeEncrypted(header=envelope_encrypted)


@app.post("/api/decrypt")
def decrypt(request: EnvelopeEncrypted) -> Dek:
    client = kms.KeyManagementServiceClient()
    envelope_encrypted = base64.urlsafe_b64decode(request.header)
    kms_response = client.decrypt(
        request={
            "name": KMS_KEY_NAME,
            "ciphertext": envelope_encrypted,
        }
    )
    envelope = Envelope.model_validate_json(kms_response.plaintext)
    # TODO: Validate that the user is allowed to decrypt
    return Dek(dek=envelope.dek, iv=envelope.iv)
