import base64
import os
from io import BytesIO

from fastapi import FastAPI
from google.cloud import kms
from pure_protobuf.annotations import Field
from pure_protobuf.message import BaseMessage
from pydantic import BaseModel
from typing_extensions import Annotated

KMS_KEY_NAME = os.environ["KMS_KEY_NAME"]


class EnvelopeEncrypted(BaseModel):
    header: str


class Envelope(BaseModel):
    dek: str
    iv: str
    authorized_users: list[str]


class EnvelopeProto(BaseMessage, BaseModel):
    dek: Annotated[bytes, Field(1)] = b""
    iv: Annotated[bytes, Field(2)] = b""
    authorized_users: Annotated[list[str], Field(3)] = []


class Dek(BaseModel):
    dek: str
    iv: str


app = FastAPI()


@app.get("/api/hello")
def read_root():
    return {"Hello": "World"}


@app.post("/api/encrypt")
def encrypt(request: Envelope) -> EnvelopeEncrypted:
    envelope_serialized = bytes(
        EnvelopeProto(
            dek=base64.b64decode(request.dek),
            iv=base64.b64decode(request.iv),
            authorized_users=request.authorized_users,
        )
    )
    print(len(envelope_serialized))

    client = kms.KeyManagementServiceClient()
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
    print(kms_response.plaintext)
    envelope = EnvelopeProto.read_from(BytesIO(kms_response.plaintext))
    # TODO: Validate that the user is allowed to decrypt
    return Dek(
        dek=base64.b64encode(envelope.dek).decode(),
        iv=base64.b64encode(envelope.iv).decode(),
    )
