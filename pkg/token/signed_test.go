package token

import "testing"

func TestVerifyFromSigned(t *testing.T) {
	signed, err := validToken.Sign(key)
	if err != nil {
		t.Fatalf("fail signing token")
	}

	err = validToken.Verify(key, signed.Signature())
	if err != nil {
		t.Fatalf("fail verify %s", err.Error())
	}
	if !signed.Parse().Equal(validToken) {
		t.Fatalf("tokens not equal")
	}
}

func TestSignFromSignedExpired(t *testing.T) {
	signed, err := expiredToken.Sign(key)
	if err != nil {
		t.Fatalf(err.Error())
	}
	err = validToken.Verify(key, signed.Signature())
	if err == nil {
		t.Fatalf("expected to fail verification")
	}
	if !signed.Parse().Equal(expiredToken) {
		t.Fatalf("tokens are not equal")
	}
}
