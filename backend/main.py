import base64
import os


from fastapi import FastAPI
from google.cloud import kms
from pydantic import BaseModel

KMS_KEY_NAME = os.environ["KMS_KEY_NAME"]


class EncryptRequest(BaseModel):
    dek: str
    authorized_users: list[str]


class EncryptResponse(BaseModel):
    envelope_header: str


class DecryptRequest(BaseModel):
    envelope_header: str


class DecryptResponse(BaseModel):
    dek: str


app = FastAPI()


@app.get("/api/hello")
def read_root():
    return {"Hello": "World"}


@app.post("/api/encrypt")
def encrypt(request: EncryptRequest) -> EncryptResponse:
    client = kms.KeyManagementServiceClient()
    plaintext = request.model_dump_json().encode()
    kms_response = client.encrypt(
        request={
            "name": KMS_KEY_NAME,
            "plaintext": plaintext,
        }
    )
    ciphertext_encoded = base64.b64encode(kms_response.ciphertext)
    return EncryptResponse(envelope_header=ciphertext_encoded)


@app.post("/api/decrypt")
def decrypt(request: DecryptRequest) -> DecryptResponse:
    client = kms.KeyManagementServiceClient()
    ciphertext = base64.b64decode(request.envelope_header)
    kms_response = client.decrypt(
        request={
            "name": KMS_KEY_NAME,
            "ciphertext": ciphertext,
        }
    )
    envelope = EncryptRequest.model_validate_json(kms_response.plaintext)
    # TODO: Validate that the user is allowed to decrypt
    return DecryptResponse(dek=envelope.dek)
