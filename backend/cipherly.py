import base64
import os
from io import BytesIO

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from google.cloud import kms
from pure_protobuf.annotations import Field
from pure_protobuf.message import BaseMessage
from pydantic import BaseModel, BeforeValidator, PlainSerializer
from starlette.exceptions import HTTPException
from typing_extensions import Annotated

KMS_KEY_NAME = os.environ["KMS_KEY_NAME"]


class EnvelopeEncrypted(BaseModel):
    header: str


Base64Bytes = Annotated[
    bytes,
    PlainSerializer(lambda v: base64.b64encode(v).decode(), str),
    BeforeValidator(lambda v: v if isinstance(v, bytes) else base64.b64decode(v)),
]


class Envelope(BaseMessage, BaseModel):
    dek: Annotated[Base64Bytes, Field(1)] = b""
    iv: Annotated[Base64Bytes, Field(2)] = b""
    authorized_users: Annotated[list[str], Field(3)] = []


class Dek(BaseModel):
    dek: Annotated[Base64Bytes, Field(1)] = b""
    iv: Annotated[Base64Bytes, Field(2)] = b""


app = FastAPI()


@app.get("/api/hello")
def read_root():
    return {"Hello": "World"}


@app.post("/api/encrypt")
def encrypt(request: Envelope) -> EnvelopeEncrypted:
    client = kms.KeyManagementServiceClient()
    kms_response = client.encrypt(
        request={
            "name": KMS_KEY_NAME,
            "plaintext": bytes(request),
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
    envelope = Envelope.read_from(BytesIO(kms_response.plaintext))
    # TODO: Validate that the user is allowed to decrypt
    return Dek(dek=envelope.dek, iv=envelope.iv)


class SPAStaticFiles(StaticFiles):
    async def get_response(self, path: str, scope):
        try:
            return await super().get_response(path, scope)
        except HTTPException as ex:
            if ex.status_code == 404:
                return await super().get_response("index.html", scope)
            else:
                raise ex


app.mount("/", SPAStaticFiles(directory="frontend", html=True), name="static")
