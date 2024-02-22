from io import BytesIO

from cipherly import Envelope


def test_json_serialization():
    actual = Envelope.model_dump(
        Envelope(dek=b"dek", iv=b"iv", authorized_users=["user1", "user2"])
    )
    expected = {
        "dek": "ZGVr",
        "iv": "aXY=",
        "authorized_users": ["user1", "user2"],
    }
    assert actual == expected


def test_json_deserialization():
    actual = Envelope.model_validate(
        {
            "dek": "ZGVr",
            "iv": "aXY=",
            "authorized_users": ["user1", "user2"],
        }
    )
    expected = Envelope(dek=b"dek", iv=b"iv", authorized_users=["user1", "user2"])
    assert actual == expected


def test_proto_serialization():
    actual = bytes(Envelope(dek=b"dek", iv=b"iv", authorized_users=["user1", "user2"]))
    expected = b"\n\x03dek\x12\x02iv\x1a\x05user1\x1a\x05user2"
    assert actual == expected


def test_proto_deserialization():
    actual = Envelope.read_from(
        BytesIO(b"\n\x03dek\x12\x02iv\x1a\x05user1\x1a\x05user2")
    )
    expected = Envelope(dek=b"dek", iv=b"iv", authorized_users=["user1", "user2"])
    assert actual == expected
